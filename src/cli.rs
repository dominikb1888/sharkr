use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sharkr", about = "A tiny packet capture tool written in Rust")]
pub struct Cli {
    /// Network interface to listen on (positional)
    #[arg(value_name = "IFACE")]
    pub iface: Option<String>,

    /// List available network interfaces
    #[arg(long)]
    pub list: bool,

    /// Write captured packets to a pcap file
    #[arg(long)]
    pub output: Option<String>,
}

