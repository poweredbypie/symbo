pub mod ghidra;
pub mod rizin;

use std::path::PathBuf;

use clap::{Args, ValueEnum};
use std::fs;

#[derive(Clone, ValueEnum)]
pub enum Backend {
    Ghidra,
    Rizin,
}

#[derive(Args)]
pub struct Generate {
    /// The backend used to generate the database.
    backend: Backend,
    proj: PathBuf,

    /// Only currently used if selected with the Ghidra backend; the path to the Ghidra installation.
    root: Option<PathBuf>,

    #[clap(short, long)]
    output: Option<PathBuf>,
}

impl Generate {
    pub fn generate(self) -> Result<(), Box<dyn std::error::Error>> {
        let out_file = self
            .output
            .or_else(|| {
                Some(PathBuf::from(
                    (self.proj.file_name()?.to_string_lossy() + ".exdb").to_string(),
                ))
            })
            .ok_or("Unable to extract filename from project path")?;
        fs::File::create(&out_file)?;

        let out_data = match self.backend {
            Backend::Rizin => rizin::generate(self.proj.display().to_string()),
            Backend::Ghidra => {
                let root = self
                    .root
                    .ok_or("Ghidra root must be set with the Ghidra backend")?;
                ghidra::generate(root, self.proj)
            }
        }?;
        fs::write(&out_file, pot::to_vec(&out_data)?)?;
        Ok(())
    }
}
