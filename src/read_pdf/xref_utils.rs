use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::models::{ObjectType, PdfObject};
use crate::utils::convert_byte_str;

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

pub fn get_obj_map(file: &mut File, start_xref_idx: u64) -> std::io::Result<(HashMap<i64, usize>, u64)> {
    let mut file_bytes = BufReader::new(file);
    let mut obj_map: HashMap<i64, usize> = HashMap::new();
    file_bytes.get_mut().rewind()?;
    file_bytes.seek(SeekFrom::Start(start_xref_idx + 5))?;
    let mut total_obj: i64 = 0;
    let mut current_obj: i64 = 0;
    let mut obj_line = String::new();
    for xref_bytes in file_bytes.by_ref().bytes() {
        let curr_str = convert_byte_str(xref_bytes);
        if curr_str == "\n" {
            let line_parts = obj_line.trim().split(' ').collect::<Vec<&str>>();
            if line_parts.len() <= 2 {
                total_obj = match line_parts[1].parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                }
            } else if line_parts.len() >= 3 {
                let byte_offset = match line_parts[0].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                };
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
                    let root_line = locate_obj_by_name(&curr_line, "root");
                    let root_obj = root_line.split(" ").collect::<Vec<&str>>();
                    if root_obj.len() >= 3 {
                        let obj_ref = root_obj[0].trim().parse::<i64>()
                            .unwrap_or_else(|_| -1);
                        let generation = root_obj[1].trim().parse::<i64>()
                            .unwrap_or_else(|_| -1);
                        if obj_ref >= 0 && generation >= 0 {
                            let pdf_obj = PdfObject::new(obj_ref, generation, true, None, ObjectType::Root);
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

fn locate_obj_by_name<'a>(obj_line: &'a str, obj_name: &'a str) -> String {
    let mut result = String::new();
    let trimmed_line = obj_line.replace("<<", "")
        .replace(">>", "").trim().to_string();
    let mut split_line = trimmed_line.split("/").collect::<Vec<&str>>();
    if split_line.len() > 0 {
        loop {
            if let Some(x) = split_line.pop() {
                if !x.is_empty() && x.to_lowercase().contains(obj_name) {
                    result = x.to_lowercase().replace(obj_name, "").trim().to_string();
                    break;
                }
            } else {
                break;
            }
        }
    }
    result
}