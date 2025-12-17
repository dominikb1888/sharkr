use pcap::{Active, Capture, Device, Savefile};
use crate::error::SharkrError;

pub fn capture(
    iface: &str,
    output: Option<&str>,
) -> Result<(), SharkrError> {
    let device = Device::list()?
        .into_iter()
        .find(|d| d.name == iface)
        .ok_or_else(|| SharkrError::InterfaceNotFound(iface.into()))?;

    let mut cap = Capture::from_device(device)?
        .promisc(true)
        .snaplen(65535)
        .open()?;

    let mut savefile: Option<Savefile> = match output {
        Some(path) => Some(cap.savefile(path)?),
        None => None,
    };

    println!("Listening on {iface}… press Ctrl+C to stop");

    while let Ok(packet) = cap.next_packet() {
        println!(
            "len={} ts={}.{}",
            packet.header.len,
            packet.header.ts.tv_sec,
            packet.header.ts.tv_usec
        );

        if let Some(ref mut sf) = savefile {
            sf.write(&packet);
        }
    }

    Ok(())
}

