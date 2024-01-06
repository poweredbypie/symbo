use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::db::ExecDB;

pub fn generate(json_path: PathBuf) -> Result<ExecDB, Box<dyn std::error::Error>> {
    // The bulk of the work is done by the Ghidra script that emits a json file
    let reader = BufReader::new(File::open(json_path)?);
    let db = serde_json::from_reader(reader)?;
    Ok(db)
}
