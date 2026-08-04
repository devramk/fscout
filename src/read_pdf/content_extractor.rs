use std::mem::replace;
use crate::utils::{convert_bytes_str, extract_obj_content, flatten_objs, get_pos_by_markers, get_ref_bytes};

const DICT_START: &[u8] = b"<<";
const DICT_END: &[u8] = b">>";
const LENGTH_DICT: &[u8] = b"/Length";
const FILTER_DICT: &[u8] = b"/Filter";

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
    let dict_end_pos = get_pos_by_markers(bytes, DICT_END)?;
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
        let filter_pos = match check_dictionary(bytes, FILTER_DICT) {
            Some(pos) => pos + FILTER_DICT.len(),
            None => return
        };
        if filter_pos > 0 {
            let filter_types = extract_filter_dictionary(bytes, filter_pos);
        }
    }
}

fn check_dictionary(bytes: &[u8], dict_type: &[u8]) -> Option<usize> {
    get_pos_by_markers(bytes, dict_type)
}

fn extract_filter_dictionary(bytes: &[u8], filter_pos: usize) -> Option<Vec<String>> {
    let mut decode_dict: Vec<String> = Vec::new();
    let mut is_filter_array = false;
    let mut pos = filter_pos;
    loop {
        if pos >= bytes.len() {
            break;
        }
        if bytes[pos] == b'\n' || bytes[pos] == b' ' || bytes[pos] == b'/' {
            pos += 1;
            continue;
        }
        if bytes[pos] == b'[' {
            is_filter_array = true;
            pos += 1;
            break;
        }
        let mut filter_dict_bytes: Vec<u8> = Vec::new();
        while pos < bytes.len() && bytes[pos] != b'/' && bytes[pos] != b'\n' {
            filter_dict_bytes.push(bytes[pos]);
            pos += 1;
        }
        if filter_dict_bytes.len() > 0 {
            decode_dict.push(convert_bytes_str(&filter_dict_bytes));
            break;
        }
        pos += 1;
    }

    if is_filter_array {
        extract_decode_types(bytes, filter_pos, &mut decode_dict);
    }
    if decode_dict.len() > 0 {
        println!("Decoded: {:#?}", decode_dict);
        return Some(decode_dict);
    }
    None
}

fn extract_decode_types(bytes: &[u8], filter_pos: usize, decode_dict: &mut Vec<String>) {
    let decode_dict_start_pos = match get_pos_by_markers(bytes, b"[") {
        Some(pos) => pos,
        None => {
            println!("Filter Decode Array Start Pos Not Found");
            return;
        }
    };
    let decode_dict_end_pos = match get_pos_by_markers(bytes, b"]") {
        Some(pos) => pos,
        None => {
            println!("Filter Decode Array End Pos Not Found");
            return;
        }
    };
    let filter_bytes = &bytes[decode_dict_start_pos..decode_dict_end_pos];
    let filter_type_str = convert_bytes_str(filter_bytes)
        .replace("\n", "")
        .replace("[", "")
        .replace("]", "")
        .replace("/", "").trim().to_string();
    let filter_type_arr = filter_type_str.split(" ").collect::<Vec<&str>>();
    for ft in filter_type_arr {
        decode_dict.push(ft.to_string());
    }
}
