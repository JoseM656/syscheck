use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "syscheck",
    version = "0.3.0",
    about = "Rust CLI tool for get system data and other utilities",
    after_help = "Use 'syscheck <command> --help' for more information about a command."
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show cpu percentage use.
    Cpu {
        /// Show CPU frequency in GHz.
        #[arg(long)]
        ghz: bool,

        /// Show all CPU info.
        #[arg(long)]
        all: bool,
    },

    /// Show memory percentage use.
    Mem {

        /// Show cache memory.
        #[arg(long)]
        cache: bool,

        /// Show swap memory.
        #[arg(long)]
        swap: bool,

        /// Show all relevant memories.
        #[arg(long)]
        all: bool,
    },

    /// Show temperature of system.
    Temp,

    /// Show devices connected in the system bus.
    Devices,

    /// Convert numbers between binary, hexadecimal, octal or decimal.
    Convert {
    
        value: String,

        #[arg(long)]
        from: String,  // "dec", "bin", "hex"

        #[arg(long)]
        to: String,    // "dec", "bin", "hex"
    }
}