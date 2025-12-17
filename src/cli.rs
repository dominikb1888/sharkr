use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "sharkr", about = "A tiny packet capture tool written in Rust")]
pub struct Cli {
    /// List available network interfaces
    #[arg(long)]
    pub list: bool,

    /// Network interface to listen on
    #[arg(long)]
    pub iface: Option<String>,

    /// Write captured packets to a pcap file
    #[arg(long)]
    pub output: Option<String>,
}

