use std::io::{BufRead, BufReader};

pub fn search_stdin(pattern: &str) -> std::io::Result<()> {

    let stdin = std::io::stdin();
    let read_buf = BufReader::new(stdin.lock());

    for line in read_buf.lines() {
        let std_text = line?;
        if std_text.to_lowercase().contains(pattern) {
            println!("{}", std_text);
        }
    }

    Ok(())
}