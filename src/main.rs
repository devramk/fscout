pub mod read_multi_file;
pub mod read_single_file;
pub mod models;
pub mod read_cmd;
mod read_pdf;
pub mod utils;

use clap::Parser;
use crate::models::Cli;
use crate::read_multi_file::traverse_all;
use crate::read_single_file::search_file;
use crate::read_cmd::search_stdin;
use crate::read_pdf::open_pdf;

fn main() -> std::io::Result<()> {
    // let pattern = args().nth(1).expect("Pattern argument not found");
    // let path = args().nth(2).expect("Path argument not found");

    let cli_args = Cli::parse();

    if cli_args.stdin {
         search_stdin(&cli_args.pattern)?;
    } else if let Some(path) = cli_args.path {
        if path.metadata()?.is_dir() {
            let file_paths = traverse_all(&path)?;
            for file_path in file_paths {
                search_file(&cli_args.pattern, &file_path)?;
            }
        } else {
            if let Some(file_name) = path.file_name() {
                if file_name.to_str().unwrap().contains(".pdf") {
                    open_pdf(&path)?;
                }
            } else {
                search_file(&cli_args.pattern, &path)?;
            }
        }
    } else {
        std::process::exit(1);
    }

    Ok(())
}
