use clap::{Parser, Subcommand};
use clap::error::ErrorKind;

mod metrics;
mod utils;

#[derive(Parser)]
#[command(
    name = "syscheck",
    version = "0.1.0",
    about = "System monitoring CLI tool"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show cpu percentage use.
    Cpu,

    /// Show memory percentage use.
    Mem,

    /// Show temperature of system.
    Temp,

    /// Show devices conected in the system bus.
    Devices,
}

fn main() {
    let cli = Cli::try_parse();

    match cli {
        Ok(cli) => match cli.command {
            Commands::Cpu => metrics::cpu::cpu(),
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