use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::models::PdfObject;

pub fn get_start_xref_idx(file: &mut File) -> std::io::Result<u64> {
    let mut start_xref_idx: u64 = 0;
    let file_len = file.metadata().map(|v| v.len()).unwrap_or(0);
    if file_len == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File is empty"));
    }
    let pos_seek = file_len.saturating_sub(1024);
    if pos_seek >= 512 {
        file.seek(SeekFrom::Start(pos_seek))?;
    } else if pos_seek < 512 {
        file.rewind()?;
    }
    let mut file_bytes = BufReader::new(file);
    let mut line_str = String::new();
    let mut startxref_found = false;
    for byte in file_bytes.by_ref().bytes() {
        let curr_str = convert_byte_str(byte);
        if curr_str == "\n" {
            if line_str.to_lowercase() == "startxref" {
                startxref_found = true;
                line_str.clear();
                continue;
            }
            if startxref_found {
                break;
            }
            line_str.clear();
        } else {
            line_str.push_str(&curr_str);
        }
    }
    if startxref_found && line_str.len() > 0 {
        start_xref_idx = line_str.trim().parse::<u64>().unwrap_or_else(|_| 0);
    }
    Ok(start_xref_idx)
}

pub fn get_obj_map(file: &mut File, start_xref_idx: u64) -> std::io::Result<(HashMap<u64, i64>, u64)> {
    let mut file_bytes = BufReader::new(file);
    let mut obj_map: HashMap<u64, i64> = HashMap::new();
    file_bytes.get_mut().rewind()?;
    file_bytes.seek(SeekFrom::Start(start_xref_idx + 5))?;
    let mut total_obj = 0;
    let mut current_obj = 0;
    let mut obj_line = String::new();
    for xref_bytes in file_bytes.by_ref().bytes() {
        let curr_str = convert_byte_str(xref_bytes);
        if curr_str == "\n" {
            let line_parts = obj_line.trim().split(' ').collect::<Vec<&str>>();
            if line_parts.len() <= 2 {
                total_obj = match line_parts[1].parse::<u64>() {
                    Ok(v) => v,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                }
            } else if line_parts.len() >= 3 {
                let byte_offset = line_parts[0].parse::<i64>().unwrap_or_else(|_| -1);
                if byte_offset >= 0 {
                    obj_map.insert(current_obj, byte_offset);
                    current_obj += 1;
                }
                if current_obj == total_obj {
                    break;
                }
            }
            obj_line.clear();
        } else {
            obj_line.push_str(&curr_str);
        }
    }
    Ok((obj_map, file_bytes.stream_position()?))
}

fn convert_byte_str(byte: std::io::Result<u8>) -> String {
    let byte = byte.unwrap_or_else(|_| 0);
    let byte_arr = [byte];
    std::str::from_utf8(&byte_arr).unwrap_or_else(|_| "").to_string()
}

pub fn parse_trailer_for_root(file: &mut File, cursor_at: u64) -> std::io::Result<PdfObject> {
    let mut file_bytes = BufReader::new(file);
    file_bytes.seek(SeekFrom::Start(cursor_at))?;
    let mut curr_line = String::new();
    let mut is_trailer_block = false;
    for byte in file_bytes.by_ref().bytes() {
        let curr_str = convert_byte_str(byte);
        if curr_str == "\n" {
            if curr_line.to_lowercase().contains("trailer") {
                is_trailer_block = true;
                curr_line.clear();
                continue;
            }
            if is_trailer_block {
                if curr_line.to_lowercase().contains("root") {
                    let root_obj = curr_line.split(" ").collect::<Vec<&str>>();
                    if root_obj.len() >= 3 {
                        let obj_ref = root_obj[1].trim().parse::<i64>()
                            .unwrap_or_else(|_| -1);
                        let generation = root_obj[2].trim().parse::<i64>()
                            .unwrap_or_else(|_| -1);
                        if obj_ref >= 0 && generation >= 0 {
                            let pdf_obj = PdfObject {
                                obj_ref,
                                generation
                            };
                            return Ok(pdf_obj);
                        }
                    }
                    break;
                }
            }
            curr_line.clear();
        } else {
            curr_line.push_str(&curr_str);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Something went wrong"))
}