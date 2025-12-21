use etherparse::{PacketHeaders, NetHeaders, TransportHeader, PayloadSlice};

#[derive(Debug)]
pub enum ParsedPacket<'a> {
    Ip {
        ip: NetHeaders,
        transport: Option<TransportHeader>,
        payload: PayloadSlice<'a>,
    },
    NonIp {
        raw: &'a [u8],
    },
}

pub fn parse_packet<'a>(data: &'a [u8], is_loopback: bool) -> ParsedPacket<'a> {
    // Skip DLT_NULL header on loopback
    let slice = if is_loopback && data.len() >= 4 {
        &data[4..]
    } else {
        data
    };

    match PacketHeaders::from_ip_slice(slice) {
        Ok(headers) => {
            if let Some(net_headers) = headers.net {
                ParsedPacket::Ip {
                    ip: net_headers,
                    transport: headers.transport,
                    payload: headers.payload,
                }
            } else {
                ParsedPacket::NonIp { raw: slice }
            }
        }
        Err(_) => ParsedPacket::NonIp { raw: slice },
    }
}

