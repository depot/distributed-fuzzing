pub const FRAME_MAGIC: &str = "DFUZ";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Frame<'a> {
    pub kind: &'a str,
    pub flags: u8,
    pub name: &'a str,
    pub body_hex: &'a str,
}

pub fn parse_frame(data: &[u8]) -> Option<Frame<'_>> {
    let text = std::str::from_utf8(data).ok()?;
    let mut parts = text.split('|');
    if parts.next()? != FRAME_MAGIC {
        return None;
    }
    if parts.next()? != "1" {
        return None;
    }
    let kind = parts.next()?;
    if !matches!(kind, "ping" | "store" | "delete") {
        return None;
    }
    let flags_text = parts.next()?;
    if flags_text.len() != 2 {
        return None;
    }
    let flags = u8::from_str_radix(flags_text, 16).ok()?;
    if flags & !0x03 != 0 {
        return None;
    }
    let name = parts.next()?;
    if name.is_empty() || name.len() > 64 {
        return None;
    }
    let body_hex = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    if !body_hex.len().is_multiple_of(2) {
        return None;
    }
    for b in body_hex.bytes() {
        decode_nibble(b)?;
    }
    match kind {
        "ping" | "delete" if !body_hex.is_empty() => return None,
        "store" if body_hex.is_empty() => return None,
        _ => {}
    }
    Some(Frame { kind, flags, name, body_hex })
}

pub fn encode_frame(frame: &Frame<'_>) -> Option<Vec<u8>> {
    if !matches!(frame.kind, "ping" | "store" | "delete") {
        return None;
    }
    if frame.flags & !0x03 != 0 {
        return None;
    }
    if frame.name.is_empty() || frame.name.len() > 64 {
        return None;
    }
    if !frame.body_hex.len().is_multiple_of(2) {
        return None;
    }
    for b in frame.body_hex.bytes() {
        decode_nibble(b)?;
    }
    match frame.kind {
        "ping" | "delete" if !frame.body_hex.is_empty() => return None,
        "store" if frame.body_hex.is_empty() => return None,
        _ => {}
    }
    Some(
        format!(
            "{FRAME_MAGIC}|1|{}|{:02x}|{}|{}",
            frame.kind, frame.flags, frame.name, frame.body_hex
        )
        .into_bytes(),
    )
}

fn decode_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
