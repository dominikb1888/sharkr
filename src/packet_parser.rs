use etherparse::PacketHeaders;

/// Represents a parsed packet
#[derive(Debug)]
pub struct ParsedPacket<'a> {
    pub ip: Option<etherparse::IpHeader>,
    pub transport: Option<etherparse::TransportHeader>,
    pub payload: Option<&'a [u8]>,
}

/// Parse a single packet captured from loopback or Ethernet.
///
/// For loopback (DLT_NULL), skip the first 4 bytes (address family header).
pub fn parse_packet(data: &[u8], is_loopback: bool) -> Result<ParsedPacket, String> {
    let packet_slice = if is_loopback {
        if data.len() < 4 {
            return Err("Packet too short to contain AF header".into());
        }
        &data[4..]
    } else {
        data
    };

    match PacketHeaders::from_ip_slice(packet_slice) {
        Ok(headers) => Ok(ParsedPacket {
            ip: headers.ip,
            transport: headers.transport,
            payload: headers.payload,
        }),
        Err(e) => Err(format!("Failed to parse packet: {:?}", e)),
    }
}

