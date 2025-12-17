use sharkr::cli;
use sharkr::capture;
use sharkr::error;
use sharkr::interfaces;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use error::SharkrError;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        interfaces::list_interfaces()?;
        return Ok(());
    }

    let iface = cli.iface.as_deref().ok_or(SharkrError::MissingInterface)?;

    capture::capture(iface, cli.output.as_deref())?;

    Ok(())
}

