pub mod read_pdf_utils;
pub mod xref_utils;
pub mod object_parser;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use crate::read_pdf::read_pdf_utils::{is_valid_pdf};
use crate::read_pdf::xref_utils::{get_start_xref_idx, get_obj_map, parse_trailer_for_root};

pub fn open_pdf(path: &PathBuf) -> std::io::Result<()> {
    let mut pdf_file = File::open(path)?;
    let byte_reader = BufReader::new(&pdf_file);
    let mut container: Vec<u8> = Vec::new();
    for byte in byte_reader.bytes() {
        container.push(byte?);
    }
    let pdf_version = is_valid_pdf(&container[0..10]);
    if pdf_version <= 0f32 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid PDF file"));
    } else if pdf_version >= 1.5 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unrecognized PDF file"));
    }
    // print_pdf_object(&container);
    let start_xref_idx = get_start_xref_idx(&mut pdf_file)?;
    let (obj_map, cursor_at) = get_obj_map(&mut pdf_file, start_xref_idx)?;
    println!("obj map: {:#?}", obj_map);
    let root_obj = parse_trailer_for_root(&mut pdf_file, cursor_at);
    println!("root_obj: {:#?}", root_obj);
    Ok(())
}