use std::path::PathBuf;

use crate::db::ExecDB;

pub fn generate(root: PathBuf, proj: PathBuf) -> Result<ExecDB, Box<dyn std::error::Error>> {
    Err("Ghidra backend not implemented".into())
}
