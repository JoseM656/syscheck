use clap::Parser;
use clap::error::ErrorKind;
use cli::{Cli, Commands};

mod cli;
mod metrics;
mod utils;


fn main() {
    let cli = Cli::try_parse();

    match cli {
        Ok(cli) => match cli.command {

            Commands::Cpu {ghz,all} => {
                metrics::cpu::cpu(ghz, all);
            },
            
            Commands::Mem => metrics::mem::mem(),
            Commands::Temp => metrics::temp::temp(),
            Commands::Devices => utils::devices::devices(),
        },

        Err(e) => match e.kind() {
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                e.print().unwrap();
            }

            _ => {
                println!("{}", e.render().to_string().lines().next().unwrap());
                println!("Use `syscheck --help` to see available commands");
            }
        },
    }
}