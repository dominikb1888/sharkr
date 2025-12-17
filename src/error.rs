use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharkrError {
    #[error("pcap error: {0}")]
    Pcap(#[from] pcap::Error),

    #[error("interface '{0}' not found")]
    InterfaceNotFound(String),

    #[error("no interface specified (use --iface)")]
    MissingInterface,
}

