use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "syscheck",
    version = "0.2.0",
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

    /// Convert a decimal, binary o hexadecimal number in any of the 3 mentioned.
    Convert {

        value: String,

        /// Convert into a binary number,
        #[arg(long)]
        to_bin: bool,

        /// Convert into a hexadecimal number.
        #[arg(long)]
        to_hex: bool,

        /// Convert into a decimal number.
        #[arg(long)]
        to_dec: bool,
    },
}