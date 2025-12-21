use sharkr::cli::Cli;
use sharkr::capture;
use sharkr::error::SharkrError;
use sharkr::interfaces;

use anyhow::Result;
use clap::Parser;

/// Ensure libpcap is usable.
/// Note: This will NOT run if libpcap.so is missing entirely,
/// because the dynamic loader will abort before main().
fn ensure_pcap_available() -> Result<(), SharkrError> {
    pcap::Capture::dead(pcap::Linktype::ETHERNET)
        .map(|_| ())
        .map_err(|err| SharkrError::PcapInitFailed(err.to_string()))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        interfaces::list_interfaces()?;
        return Ok(());
    }

    let iface = cli
        .iface
        .as_deref()
        .ok_or(SharkrError::MissingInterface)?;

    // Fail early with a helpful message if pcap is broken
    ensure_pcap_available()?;

    capture::capture(iface, cli.output.as_deref())?;

    Ok(())
}

