use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub pattern: String,
    #[arg(value_name = "PATH")]
    pub path: Option<std::path::PathBuf>,
    #[arg(short, long)]
    pub stdin: bool
}

#[derive(PartialEq, Debug)]
pub enum ObjectType {
    Pages,
    Page,
    Kids,
    Contents,
    Catalog,
    Root,
    Count,
    NA
}

#[derive(Debug, PartialEq)]
pub struct PdfObject {
    pub obj_ref: i64,
    pub generation: i64,
    pub is_ref: bool,
    pub kids: Vec<i64>,
    pub object_type: ObjectType
}

impl PdfObject {
    pub fn new(obj_ref: i64, generation: i64, is_ref: bool, kids: Option<Vec<i64>>, object_type: ObjectType) -> Self {

        Self {
            obj_ref,
            generation,
            is_ref,
            kids: kids.unwrap_or_default(),
            object_type
        }
    }
}