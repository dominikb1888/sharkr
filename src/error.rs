use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharkrError {
    #[error("pcap error: {0}")]
    Pcap(#[from] pcap::Error),

    #[error("interface '{0}' not found")]
    InterfaceNotFound(String),

    #[error("no interface specified (use --iface)")]
    MissingInterface,

    #[error(
        "libpcap could not be initialized ({0}).\n\n\
        Make sure libpcap is installed:\n\
          Debian/Ubuntu: sudo apt install libpcap1\n\
          Fedora:        sudo dnf install libpcap\n\
          Arch:          sudo pacman -S libpcap\n\n\
        If cross-compiling, ensure the target system has libpcap installed."
    )]
    PcapInitFailed(String),
}

