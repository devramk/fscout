use crate::utils::convert_bytes_str;

/**
4 0 obj
<<
/PageMode /UseNone /Pages 6 0 R /Type /Catalog
>>
endobj
**/
pub fn get_object(bytes: &[u8], start_idx: usize, end_idx: usize) {
    let mut obj_bytes = bytes[start_idx..=end_idx].to_vec();
    let content_bytes = match extract_obj_content(&obj_bytes) {
        Some(content_bytes) => content_bytes,
        None => return,
    };
    
}

fn extract_obj_content(bytes: &[u8]) -> Option<&[u8]> {
    let bytes_obj = b"obj";
    let bytes_end_obj = b"endobj";
    let start_pos = bytes.windows(bytes_obj.len()).position(|p| p == bytes_obj)?;
    let content_start = start_pos + bytes_obj.len();
    let end_pos = bytes.windows(bytes_end_obj.len()).position(|p| p == bytes_end_obj)?;
    let content_end = content_start + end_pos;
    
    Some(&bytes[content_start..content_end])
}

fn flaten_objs(bytes: &[u8]) {

}