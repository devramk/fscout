use crate::utils::{convert_bytes_str, extract_obj_content, flatten_objs, get_pos_by_markers, get_ref_bytes};

const DICT_START: &[u8] = b"<<";
const DICT_END: &[u8] = b">>";
const LENGTH_DICT: &[u8] = b"/Length";

pub fn get_content_object(bytes: &[u8], start_idx: usize) {
    let obj_bytes = bytes[start_idx..].to_vec();
    let content_bytes = match extract_obj_content(&obj_bytes) {
        Some(cb) => cb,
        None => return
    };
    let dictionary_bytes = match get_dictionary(content_bytes) {
        Some(dict_bytes) => dict_bytes,
        None => return
    };
    let skimmed_dict_bytes = flatten_objs(dictionary_bytes);
    println!("{}", convert_bytes_str(&skimmed_dict_bytes));
    parse_dictionary(&skimmed_dict_bytes);
}

fn get_dictionary(bytes: &[u8]) -> Option<&[u8]> {
    let dict_start_pos = get_pos_by_markers(bytes, DICT_START)? + DICT_START.len();
    let dict_end_pos = dict_start_pos + get_pos_by_markers(bytes, DICT_END)?;
    Some(&bytes[dict_start_pos..dict_end_pos])
}

fn parse_dictionary(bytes: &[u8]) {
    let length_dict_pos = match check_dictionary(bytes, LENGTH_DICT) {
        Some(pos) => pos + LENGTH_DICT.len(),
        None => return
    };
    let mut len_bytes: Vec<u8> = Vec::new(); 
    get_ref_bytes(bytes, length_dict_pos, &mut len_bytes);
    if len_bytes.len() > 0 {
        let content_len: i64 = match convert_bytes_str(len_bytes.as_slice()).parse::<i64>() {
            Ok(l) => l,
            Err(_) => return
        };
        println!("Content Length: {}", content_len);
    }
}

fn check_dictionary(bytes: &[u8], dict_type: &[u8]) -> Option<usize> {
    get_pos_by_markers(bytes, dict_type)
}
