use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::db::ExecDB;

pub fn generate(json_path: PathBuf) -> Result<ExecDB, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(json_path)?);
    let db = serde_json::from_reader(reader)?;
    Ok(db)
}
