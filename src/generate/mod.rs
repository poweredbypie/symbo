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
            Backend::Ghidra => ghidra::generate(self.proj),
        }?;
        fs::write(&out_file, pot::to_vec(&out_data)?)?;
        Ok(())
    }
}
