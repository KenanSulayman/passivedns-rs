//! DNS packet struct and implementation
use header::Header;
use payload::Payload;

#[derive(Debug)]
pub struct Response<'a> {
    pub header: Header,
    pub payload: Payload<'a>,
}

impl<'a> Response<'a> {
    pub fn new(data: &'a[u8]) -> Option<Response<'a>> {
        if data.len() < 0x36 {
            return None;
        }
        if !Response::is_udp(data) {
            return None;
        }
        // if not port 53
        if !Response::is_port_53(data) {
            return None;
        }
        let hdr = Header::new(&data[0x2a..0x36]);
        // if question_response is not a response
        if !hdr.qr {
            return None;
        }
        let payload = Payload::new(&hdr, &data[0x36..]);
        Some(Response {
            header: hdr,
            payload: payload,
        })
    }

    fn is_udp(data: &[u8]) -> bool {
        let proto_bytes = &data[0x17];
        *proto_bytes == 0x11
    }

    fn is_port_53(data: &[u8]) -> bool {
        let src_port_bytes = &data[0x22..0x24];
        let src_port: u16 = (u16::from(src_port_bytes[0]) << 8) + u16::from(src_port_bytes[1]);
        src_port == 53
    }

    pub fn records(&self) -> Vec<String> {
        self.payload.records()
    }
}
