use regex::Regex;
use crate::models::{ObjectType, PdfObject};
use crate::utils::{convert_bytes_str, get_pos_by_markers, extract_obj_content, flatten_objs, get_ref_bytes, CATALOG, CONTENTS, PAGES, PAGE, KIDS, COUNT};

/** extract obj contents > check if the object is valid (eg: if root obj, check if catalog is present
* if pages, check for counts and kids, if page check for contents) > get child obj reference.
* if child is stream or actual content, extract them.
*/

pub fn get_object(bytes: &[u8], start_idx: usize, pdf_object: &PdfObject) -> Option<PdfObject> {
    let obj_bytes = bytes[start_idx..].to_vec();
    let content_bytes = match extract_obj_content(&obj_bytes) {
        Some(content_bytes) => content_bytes,
        None => {
            println!("Object is corrupted");
            return None;
        },
    };
    let skimmed_bytes = flatten_objs(content_bytes);
    if !check_valid_obj(skimmed_bytes.as_slice(), &pdf_object.object_type) {
        return None;
    }
    get_child_obj_ref(skimmed_bytes.as_slice(), &look_for(&pdf_object.object_type))
}

fn check_valid_obj(bytes: &[u8], obj_type: &ObjectType) -> bool {
    match obj_type {
        ObjectType::Root => {
            check_type_present(bytes, get_byte_types(look_for(&ObjectType::Root)).as_slice())
                && check_type_present(bytes, get_byte_types(look_for(&ObjectType::Catalog)).as_slice())
        }
        ObjectType::Catalog => {
            check_type_present(bytes, get_byte_types(look_for(&ObjectType::Catalog)).as_slice())
        }
        ObjectType::Pages => {
            check_type_present(bytes, get_byte_types(look_for(&ObjectType::Pages)).as_slice())
                && check_type_present(bytes, get_byte_types(look_for(&ObjectType::Kids)).as_slice())
        }
        ObjectType::Page => {
            check_type_present(bytes, get_byte_types(look_for(&ObjectType::Page)).as_slice())
        }
        ObjectType::Contents => {
            check_type_present(bytes, get_byte_types(look_for(&ObjectType::Page)).as_slice())
        }
        _ => false
    }
}

fn check_type_present(bytes: &[u8], type_bytes: &[u8]) -> bool {
    match get_pos_by_markers(bytes, type_bytes) {
        Some(pos) => pos > 0,
        None => false,
    }
}

fn look_for(obj_type: &ObjectType) -> ObjectType {
    match obj_type {
        ObjectType::Root => ObjectType::Catalog,
        ObjectType::Catalog => ObjectType::Pages,
        ObjectType::Pages => ObjectType::Kids,
        ObjectType::Kids => ObjectType::Count,
        ObjectType::Page => ObjectType::Contents,
        _ => ObjectType::NA
    }
}

fn get_byte_types(obj_type: ObjectType) -> Vec<u8> {
    match obj_type {
        ObjectType::Catalog => CATALOG.to_vec(),
        ObjectType::Pages => PAGES.to_vec(),
        ObjectType::Kids => KIDS.to_vec(),
        ObjectType::Contents => CONTENTS.to_vec(),
        ObjectType::Count => COUNT.to_vec(),
        ObjectType::Page => PAGE.to_vec(),
        _ => vec![]
    }
}

fn get_child_obj_ref(bytes: &[u8], obj_type: &ObjectType) -> Option<PdfObject> {
    let child_ref: Option<Vec<u8>> = match obj_type {
        ObjectType::Catalog => {
            extract_child_obj_ref(bytes, PAGES)
        },
        ObjectType::Contents => {
            extract_child_obj_ref(bytes, CONTENTS)
        },
        _ => None
    };

    if let Some(child_ref) = child_ref {
        let child_obj_str = convert_bytes_str(child_ref.as_slice());
        let (obj_ref, gen_no, is_ref) = match parse_obj_ref_str(&child_obj_str) {
            Some((obj_ref, gen_no, is_ref)) => (obj_ref, gen_no, is_ref),
            None => return None
        };
        let child_obj = PdfObject::new(obj_ref, gen_no, is_ref, None, *obj_type);
        return Some(child_obj);
    }

    None
}

fn parse_obj_ref_str(obj_ref_str: &String) -> Option<(i64, i64, bool)> {
    let regex = match Regex::new(r"^(\d+) (\d+) ([A-Za-z])$") {
        Ok(regex) => regex,
        Err(_) => return None
    };
    let captures = regex.captures(obj_ref_str)?;
    let obj_ref: i64 = captures.get(1)?.as_str().parse().ok()?;
    let gen_no : i64 = captures.get(2)?.as_str().parse().ok()?;
    let is_ref: bool = captures.get(3)?.as_str().to_lowercase() == "r";
    Some((obj_ref, gen_no, is_ref))
}

fn extract_child_obj_ref(bytes: &[u8], type_bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() == 0 || type_bytes.len() == 0 {
        return None;
    }

    let child_type_pos = get_pos_by_markers(bytes, type_bytes)? + type_bytes.len();
    let mut child_obj_bytes: Vec<u8> = Vec::new();
    get_ref_bytes(bytes, child_type_pos, &mut child_obj_bytes);
    if child_obj_bytes.len() >= 1 {
        return Some(child_obj_bytes);
    }
    None
}

fn extract_pages_obj_ref(bytes: &[u8]) -> Option<Vec<PdfObject>> {
    if bytes.len() == 0 {
        return None;
    }
    let count_pos = get_pos_by_markers(bytes, COUNT)? + COUNT.len();
    let mut count_bytes: Vec<u8> = Vec::new();
    get_ref_bytes(bytes, count_pos, &mut count_bytes);
    if count_bytes.len() >= 1 {
        let count: i64 = match convert_bytes_str(&count_bytes).parse::<i64>() {
            Ok(count) => count,
            Err(_) => return None
        };
        if count > 0 {
            let kids_pos = get_pos_by_markers(bytes, KIDS)? + KIDS.len();
            let mut kids_bytes: Vec<u8> = Vec::new();
            get_ref_bytes(bytes, kids_pos, &mut kids_bytes);
            let kids_str = convert_bytes_str(kids_bytes.as_slice())
                .replace("[", "")
                .replace("]", "").trim().to_string();
            println!("{}", kids_str);
            let kids_ref = parse_kids_obj_ref(&kids_str);
            let mut kids_objs: Vec<PdfObject> = Vec::new();
            for kr in kids_ref {
                let (obj_ref, gen_no, is_ref) = kr;
                let obj = PdfObject::new(obj_ref, gen_no, is_ref, None, ObjectType::Page);
                kids_objs.push(obj);
            }
            return Some(kids_objs);
        }
    }
    None
}

fn parse_kids_obj_ref(kids_obj_ref_str: &String) -> Vec<(i64, i64, bool)> {
    let regex = Regex::new(r"(\d+)\s+(\d+)\s+(\w+)").unwrap();
    regex.captures_iter(kids_obj_ref_str)
        .filter_map(|caps| {
            let obj_ref: i64 = caps.get(1)?.as_str().parse().ok()?;
            let gen_no : i64 = caps.get(2)?.as_str().parse().ok()?;
            let is_ref: bool = caps.get(3)?.as_str().to_lowercase() == "r";
            Some((obj_ref, gen_no, is_ref))
        }).collect()
}

pub fn get_pages_obj(bytes: &[u8], start_idx: usize, pdf_object: &PdfObject) -> Option<Vec<PdfObject>> {
    let obj_bytes = bytes[start_idx..].to_vec();
    let content_bytes = match extract_obj_content(&obj_bytes) {
        Some(content_bytes) => content_bytes,
        None => {
            println!("Object is corrupted");
            return None;
        },
    };
    let skimmed_bytes = flatten_objs(content_bytes);
    if !check_valid_obj(skimmed_bytes.as_slice(), &pdf_object.object_type) {
        return None;
    }
    extract_pages_obj_ref(&skimmed_bytes)
}