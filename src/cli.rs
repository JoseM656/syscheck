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

    /// Show temperature of system. For now are cpu sensors, after time will be gpu also. 
    Temp{

        /// Shows ALL sensors.
        #[arg(long)]
        all:bool,

    },

    /// Show devices connected in the system bus.
    Devices,

    /// Convert numbers between binary, hexadecimal, octal or decimal.
    Convert {
    
        value: String,

        /// The current number system.
        #[arg(long)]
        from: String,  

        /// The expected number system.
        #[arg(long)]
        to: String,    
    },

    /// Prints into terminal information about the system.
    Info
}