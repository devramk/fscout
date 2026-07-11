use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn get_startxref_idx(file: &mut File) -> std::io::Result<u64> {
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

pub fn get_obj_map(file: &mut File, start_xref_idx: u64) -> std::io::Result<HashMap<u64, u64>> {
    let mut file_bytes = BufReader::new(file);
    let mut obj_map: HashMap<u64, u64> = HashMap::new();
    file_bytes.get_mut().rewind()?;
    file_bytes.seek(SeekFrom::Start(start_xref_idx + 5))?;
    let mut total_obj = 0;
    let mut current_obj = 1;
    let mut obj_line = String::new();
    for xref_bytes in file_bytes.by_ref().bytes() {
        if current_obj == total_obj {
            break;
        }
        let curr_str = convert_byte_str(xref_bytes);
        if curr_str == "\n" {
            let line_parts = obj_line.trim().split(' ').collect::<Vec<&str>>();
            if line_parts.len() <= 2 {
                total_obj = line_parts[1].parse::<u64>().unwrap_or_else(|_| 0);
            } else if line_parts.len() >= 3 {
                let byte_offset = line_parts[0].parse::<u64>().unwrap_or_else(|_| 0);
                if byte_offset != 0 {
                    obj_map.insert(current_obj, byte_offset);
                    current_obj += 1;
                }
            }
            obj_line.clear();
        } else {
            obj_line.push_str(&curr_str);
        }
    }
    Ok(obj_map)
}

fn convert_byte_str(byte: std::io::Result<u8>) -> String {
    let byte = byte.unwrap_or_else(|_| 0);
    let byte_arr = [byte];
    std::str::from_utf8(&byte_arr).unwrap_or_else(|_| "").to_string()
}