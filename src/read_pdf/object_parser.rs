use crate::utils::{convert_bytes_str, get_pos_by_markers};

/**
4 0 obj
<<
/PageMode /UseNone /Pages 6 0 R /Type /Catalog
>>
endobj
**/
pub fn get_object(bytes: &[u8], start_idx: usize, end_idx: usize) {
    let obj_bytes = bytes[start_idx..=end_idx].to_vec();
    let content_bytes = match extract_obj_content(&obj_bytes) {
        Some(content_bytes) => content_bytes,
        None => {
            println!("Object is corrupted");
            return;
        },
    };
    let skimmed_bytes = flaten_objs(content_bytes);
    let skimmed_byte_str = convert_bytes_str(&skimmed_bytes);
    let trimmed_str = skimmed_byte_str.trim();
    println!("Obj str {}", trimmed_str );
}

fn extract_obj_content(bytes: &[u8]) -> Option<&[u8]> {
    let bytes_obj = b"obj";
    let bytes_end_obj = b"endobj";
    let content_start = get_pos_by_markers(bytes, bytes_obj)? + bytes_obj.len();
    let content_end = content_start + get_pos_by_markers(bytes, bytes_end_obj)?;
    
    Some(&bytes[content_start..content_end])
}

fn flaten_objs(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| if b == b'\n' || b == b'<' || b == b'>' { b' ' } else { b }).collect()
}