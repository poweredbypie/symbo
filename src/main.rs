use std::fs;
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

            let mut binds = if let Some(ref out) = output {
                serde_json::from_slice(&std::fs::read(out).unwrap()).expect("Invalid symdb file")
            } else {
                BindDB::new(&pair)
            };

            println!("To do!");

            let out_file = output.clone().unwrap_or(PathBuf::from("symbols.symdb"));

            binds.process(analysis::string_xref_strat(&pair, &binds), &out_file);
            binds.process(analysis::call_xref_strat(&pair, &binds), &out_file);
            binds.process(analysis::call_block_strat(&pair, &binds), &out_file);
        }

        Command::Print { exec, addr } => {
            let exec: ExecDB =
                pot::from_slice(&std::fs::read(exec).unwrap()).expect("Invalid exdb file");
            println!("{:#?}", exec.fns.get(&addr));
        }
    }
}
