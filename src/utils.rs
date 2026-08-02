pub fn convert_bytes_str(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

pub fn convert_byte_str(byte: std::io::Result<u8>) -> String {
    match byte {
        Ok(b'\n') => return "\n".to_string(),
        Ok(b'\r') => return "\r".to_string(),
        Ok(b'\t') => return "\t".to_string(),
        Ok(b' ') => return " ".to_string(),
        Err(_) => return "".to_string(),
        Ok(_) => {}
    }

    let byte = byte.unwrap_or_else(|_| 0);
    let byte_arr = [byte];
    std::str::from_utf8(&byte_arr).unwrap_or_else(|_| "").trim().to_string()
}

pub fn is_new_ln(byte: &std::io::Result<u8>) -> bool {
    match byte {
        Ok(byte) => *byte == b'\n',
        Err(_) => false,
    }
}

pub fn get_pos_by_markers(bytes: &[u8], marker: &[u8]) -> Option<usize> {
    bytes.windows(marker.len()).position(|p| p == marker)
}