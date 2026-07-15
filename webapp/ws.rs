use base64::Engine;

const MAGIC: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub const OPCODE_TEXT: u8 = 0x1;
pub const OPCODE_CLOSE: u8 = 0x8;
pub const OPCODE_PING: u8 = 0x9;
pub const OPCODE_PONG: u8 = 0xA;

pub fn accept_key(client_key: &str) -> String {
    let combined = format!("{client_key}{MAGIC}");
    let digest = super::sha1::sha1(combined.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(digest)
}

pub fn encode_text_frame(payload: &str) -> Vec<u8> {
    encode_frame(OPCODE_TEXT, payload.as_bytes())
}

pub fn encode_frame(opcode: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + 10);
    out.push(0x80 | (opcode & 0x0F));
    let len = payload.len();
    if len < 126 {
        out.push(len as u8);
    } else if len <= 0xFFFF {
        out.push(126);
        out.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        out.push(127);
        out.extend_from_slice(&(len as u64).to_be_bytes());
    }
    out.extend_from_slice(payload);
    out
}

pub struct DecodedFrame {
    pub opcode: u8,
    pub payload: Vec<u8>,
    pub consumed: usize,
}

pub fn decode_frame(buf: &[u8]) -> Option<DecodedFrame> {
    if buf.len() < 2 {
        return None;
    }
    let opcode = buf[0] & 0x0F;
    let masked = buf[1] & 0x80 != 0;
    let len7 = (buf[1] & 0x7F) as usize;

    let mut pos = 2usize;
    let payload_len: usize = if len7 == 126 {
        if buf.len() < pos + 2 {
            return None;
        }
        let l = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        l
    } else if len7 == 127 {
        if buf.len() < pos + 8 {
            return None;
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&buf[pos..pos + 8]);
        pos += 8;
        u64::from_be_bytes(arr) as usize
    } else {
        len7
    };

    let mask_key = if masked {
        if buf.len() < pos + 4 {
            return None;
        }
        let key = [buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]];
        pos += 4;
        Some(key)
    } else {
        None
    };

    if buf.len() < pos + payload_len {
        return None;
    }
    let mut payload = buf[pos..pos + payload_len].to_vec();
    if let Some(key) = mask_key {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= key[i % 4];
        }
    }
    pos += payload_len;

    Some(DecodedFrame { opcode, payload, consumed: pos })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_key_matches_rfc6455_example() {
        assert_eq!(accept_key("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }

    #[test]
    fn masked_client_frame_round_trips_through_decode() {
        let payload = b"hello";
        let mask_key = [0x11u8, 0x22, 0x33, 0x44];
        let mut frame = vec![0x80 | OPCODE_TEXT, 0x80 | (payload.len() as u8)];
        frame.extend_from_slice(&mask_key);
        for (i, &b) in payload.iter().enumerate() {
            frame.push(b ^ mask_key[i % 4]);
        }
        let decoded = decode_frame(&frame).unwrap();
        assert_eq!(decoded.opcode, OPCODE_TEXT);
        assert_eq!(decoded.payload, payload);
        assert_eq!(decoded.consumed, frame.len());
    }
}
