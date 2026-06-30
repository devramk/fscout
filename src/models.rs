use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub pattern: String,
    #[arg(value_name = "PATH")]
    pub path: Option<std::path::PathBuf>,
    #[arg(short, long)]
    pub stdin: bool
}