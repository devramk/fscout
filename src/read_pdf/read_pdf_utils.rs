pub fn is_valid_pdf(bytes: &[u8]) -> f32 {
    if bytes.is_empty() {
        return -1f32;
    }
    let mut first_line_bytes: Vec<u8> = Vec::new();
    for byte in bytes {
        if *byte == b'\n' {
            break;
        }
        first_line_bytes.push(*byte);
    }
    let first_line = match std::str::from_utf8(&first_line_bytes) {
        Ok(v) => v,
        Err(_) => return -1f32,
    };
    let version = first_line.split("-")
        .collect::<Vec<&str>>()[1]
        .trim().parse::<f32>()
        .unwrap_or_else(|_| -1f32);
    if version > 0f32 {
        return version;
    }
    -1f32
}