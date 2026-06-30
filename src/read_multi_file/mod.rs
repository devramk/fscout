use std::fs::read_dir;
use std::path::PathBuf;

pub fn traverse_all(path: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
    println!("Traversing all files in the directory: {:?}", path);

    let mut paths:Vec<PathBuf> = Vec::new();

    for entry in read_dir(path)? {
        let item_in_dir = entry?;
        if item_in_dir.file_type()?.is_file()
            && !item_in_dir.file_name().to_str().unwrap().starts_with(".") {
            paths.push(item_in_dir.path());
        }
    }
    Ok(paths)
}