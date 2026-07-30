use crate::utils::convert_bytes_str;

/**
4 0 obj
<<
/PageMode /UseNone /Pages 6 0 R /Type /Catalog
>>
endobj
**/
pub fn get_object(bytes: &[u8], start_idx: usize, end_idx: usize) {
    let (new_ln, gr_sym, le_sym) = (b'\n', b'<', b'>');
    let (type_char, space_char, mut idx) = (b'/', b' ', 0usize);
    let obj_bytes = bytes[start_idx..=end_idx].to_vec();

    loop {
        if idx >= obj_bytes.len() {
            break;
        }

        if obj_bytes[idx] == gr_sym || obj_bytes[idx] == le_sym || obj_bytes[idx] == new_ln {
            idx += 1;
            continue;
        }

        let mut curr_ln_bytes: Vec<u8> = Vec::new();
        while idx < obj_bytes.len() && obj_bytes[idx] != new_ln {
            curr_ln_bytes.push(obj_bytes[idx]);
            idx += 1;
        }
        if curr_ln_bytes.len() > 0 {
            check_ln_objs(&curr_ln_bytes);
        }
        println!("curr ln bytes: {:#?}", curr_ln_bytes);

        idx += 1;
    }
}

fn check_ln_objs(bytes: &[u8]) {
    let ln_str = convert_bytes_str(bytes);
    
}