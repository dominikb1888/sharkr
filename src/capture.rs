use pcap::{Capture, Device};
use crate::error::SharkrError;
use crate::packet_parser::{parse_packet};

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

    let is_loopback = matches!(cap.get_datalink(), pcap::Linktype(0));

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                match parse_packet(&packet.data, is_loopback) {
                    Ok(parsed) => {
                        println!("{:?}", parsed);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => return Err(e.into()),        }
    }
}

