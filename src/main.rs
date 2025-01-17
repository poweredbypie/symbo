use colored::Colorize;
use std::path::PathBuf;

mod analysis;
mod db;
mod generate;
mod pipes;
mod util;

use crate::db::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Symbo")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate(generate::Generate),
    Run {
        from: PathBuf,
        to: PathBuf,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    Print {
        exec: PathBuf,
        addr: u64,
    },
    /// Remove unverified symbols from symdb
    Strip {
        file: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Generate(gen) => {
            gen.generate().unwrap();
        }
        Command::Run { from, to, output } => {
            let pair = ExecPair {
                input: pot::from_slice(&std::fs::read(from).unwrap()).expect("Invalid exdb file"),
                output: pot::from_slice(&std::fs::read(to).unwrap()).expect("Invalid exdb file"),
            };

            let file_path = output.unwrap_or(PathBuf::from("symbols.symdb"));

            let mut binds = if file_path.exists() {
                serde_json::from_slice(&std::fs::read(&file_path).unwrap())
                    .expect("Invalid symdb file")
            } else {
                BindDB::new(&pair)
            };

            println!("To do!");

            //binds.process(analysis::string_xref_strat(&pair, &binds), &out_file);
            //binds.process(analysis::call_xref_strat(&pair, &binds), &file_path);
            //binds.process(analysis::call_block_strat(&pair, &binds), &out_file);
            binds.process(analysis::block_traverse_strat(&pair, &binds), &file_path);
        }

        Command::Strip { file } => {
            let mut binds: BindDB =
                serde_json::from_slice(&std::fs::read(&file).unwrap()).expect("Invalid symdb file");
            let before_count = binds.binds.len();

            binds.binds.retain(|_, x| !matches!(x, Bind::Unverified(_)));

            println!(
                "Removed {} symbols",
                (before_count - binds.binds.len())
                    .to_string()
                    .bright_green()
            );

            std::fs::write(file, serde_json::to_string_pretty(&binds).unwrap()).unwrap();
        }

        Command::Print { exec, addr } => {
            let exec: ExecDB =
                pot::from_slice(&std::fs::read(exec).unwrap()).expect("Invalid exdb file");
            println!("{:#?}", exec.fns.get(&addr));
        }
    }
}
