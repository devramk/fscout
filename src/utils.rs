pub const OBJ_START_MARKER: &[u8] = b"obj";
pub const OBJ_END_MARKER: &[u8] = b"endobj";
pub const CATALOG: &[u8] = b"/Catalog";
pub const PAGES: &[u8] = b"/Pages";
pub const PAGE: &[u8] = b"/Page";
pub const KIDS: &[u8] = b"/Kids";
pub const COUNT: &[u8] = b"/Count";
pub const CONTENTS: &[u8] = b"/Contents";
pub const STREAM_START: &[u8] = b"stream";
pub const STREAM_END: &[u8] = b"endstream";

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

pub fn extract_obj_content(bytes: &[u8]) -> Option<&[u8]> {
    let content_start = get_pos_by_markers(bytes, OBJ_START_MARKER)? + OBJ_START_MARKER.len();
    let content_end = content_start
        + get_pos_by_markers(&bytes[content_start..], OBJ_END_MARKER)?;
    Some(&bytes[content_start..content_end])
}

pub fn flatten_objs(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| if b == b'\n' || b == b'<' || b == b'>' { b' ' } else { b }).collect()
}

pub fn get_ref_bytes(bytes: &[u8], mut child_type_pos: usize, child_obj_bytes: &mut Vec<u8>) {
    loop {
        if child_type_pos >= bytes.len() || bytes[child_type_pos] == b'/' {
            break;
        }
        if bytes[child_type_pos] == b'\n' || bytes[child_type_pos] == b' ' {
            child_type_pos += 1;
            continue;
        }
        while child_type_pos < bytes.len() && bytes[child_type_pos] != b'/' {
            child_obj_bytes.push(bytes[child_type_pos]);
            child_type_pos += 1;
        }
    }
}