use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn search_file(pattern: &str, path: &PathBuf) -> std::io::Result<()> {

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err)
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let text = line?;
        if text.to_lowercase().contains(pattern.to_lowercase().as_str()) {
            println!("{} -> {}", path.file_name().unwrap().to_str().unwrap(), text);
        }
    }

    Ok(())
}