use pcap::{Capture, Device};
use crate::error::SharkrError;

pub fn capture(
    iface: &str,
    _output: Option<&str>,
) -> Result<(), SharkrError> {
    let device = Device::list()?
        .into_iter()
        .find(|d| d.name == iface)
        .ok_or_else(|| SharkrError::InterfaceNotFound(iface.into()))?;

    let inactive = Capture::from_device(device)?
        .snaplen(65535)
        .timeout(1)
        .immediate_mode(true);

    let mut cap = inactive.open()?;
    cap.direction(pcap::Direction::InOut)?;

    println!("DLT: {:?}", cap.get_datalink());

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                println!(
                    "len={} ts={}.{} data={:?}",

                    packet.header.len,
                    packet.header.ts.tv_sec,
                    packet.header.ts.tv_usec,
                    packet.data,
                );
            }
            Err(pcap::Error::TimeoutExpired) => {
                // normal — ignore
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

