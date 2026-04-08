pub const PALETTE: &[u32] = &[
    0x4C72B0, 0xDD8452, 0x55A868, 0xC44E52, 0x8172B3,
    0x64B5CD, 0xDA8BC3, 0xCCB974, 0x937860, 0x8C8C8C,
];

#[inline(always)]
pub fn palette_color(custom: &[u32], i: usize) -> u32 {
    let p = if custom.is_empty() { PALETTE } else { custom };
    p[i % p.len()]
}

#[inline(always)]
pub fn push(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(s.as_bytes());
}

#[inline(always)]
pub fn push_b(buf: &mut Vec<u8>, b: &[u8]) {
    buf.extend_from_slice(b);
}

pub fn hex6(c: u32) -> [u8; 6] {
    let h = b"0123456789abcdef";
    [
        h[((c >> 20) & 0xF) as usize], h[((c >> 16) & 0xF) as usize],
        h[((c >> 12) & 0xF) as usize], h[((c >> 8) & 0xF) as usize],
        h[((c >> 4) & 0xF) as usize],  h[(c & 0xF) as usize],
    ]
}

#[inline]
pub fn push_hex(buf: &mut Vec<u8>, c: u32) {
    buf.push(b'#');
    buf.extend_from_slice(&hex6(c));
}

pub fn escape_xml(buf: &mut Vec<u8>, s: &str) {
    for ch in s.chars() {
        match ch {
            '&'  => buf.extend_from_slice(b"&amp;"),
            '<'  => buf.extend_from_slice(b"&lt;"),
            '>'  => buf.extend_from_slice(b"&gt;"),
            '"'  => buf.extend_from_slice(b"&quot;"),
            '\'' => buf.extend_from_slice(b"&#39;"),
            c    => { let mut tmp = [0u8; 4]; buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes()); }
        }
    }
}

pub fn escape_xml_s(s: &str) -> String {
    let mut buf = Vec::with_capacity(s.len() + 8);
    escape_xml(&mut buf, s);
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max { s } else { &s[..max] }
}

pub fn write_f2(buf: &mut Vec<u8>, v: f64) {
    let s = format!("{:.2}", v);
    buf.extend_from_slice(s.as_bytes());
}

pub fn write_f1(buf: &mut Vec<u8>, v: f64) {
    let s = format!("{:.1}", v);
    buf.extend_from_slice(s.as_bytes());
}

#[inline]
pub fn write_i(buf: &mut Vec<u8>, v: i32) {
    let s = itoa_i32(v);
    buf.extend_from_slice(s.as_bytes());
}

#[inline(always)]
pub fn push_i(buf: &mut Vec<u8>, mut n: i32) {
    if n < 0 { buf.push(b'-'); n = -n; }
    if n == 0 { buf.push(b'0'); return; }
    let mut d = [0u8; 10]; let mut len = 0;
    while n > 0 { d[len] = (n % 10) as u8 + b'0'; n /= 10; len += 1; }
    for &b in d[..len].iter().rev() { buf.push(b); }
}

#[inline(always)]
pub fn push_f2(buf: &mut Vec<u8>, v: f64) {
    let neg = v < 0.0;
    let vi = (v.abs() * 100.0 + 0.5) as u64;
    let whole = vi / 100; let frac = vi % 100;
    if neg { buf.push(b'-'); }
    let mut d = [0u8; 20]; let mut len = 0; let mut w = whole;
    if w == 0 { d[0] = b'0'; len = 1; } else { while w > 0 { d[len] = (w % 10) as u8 + b'0'; w /= 10; len += 1; } }
    for &b in d[..len].iter().rev() { buf.push(b); }
    buf.push(b'.'); buf.push((frac / 10) as u8 + b'0'); buf.push((frac % 10) as u8 + b'0');
}

fn itoa_i32(mut n: i32) -> String {
    if n == 0 { return "0".to_string(); }
    let neg = n < 0;
    if neg { n = -n; }
    let mut digits = [0u8; 11];
    let mut pos = 11;
    while n > 0 {
        pos -= 1;
        digits[pos] = (n % 10) as u8 + b'0';
        n /= 10;
    }
    if neg { pos -= 1; digits[pos] = b'-'; }
    String::from_utf8_lossy(&digits[pos..]).to_string()
}

pub fn lerp_color(t: f64, low: u32, mid: u32, high: u32) -> u32 {
    let (ca, cb, u) = if t <= 0.5 {
        (low, mid, t * 2.0)
    } else {
        (mid, high, (t - 0.5) * 2.0)
    };
    let lerp_ch = |shift: u32| -> u32 {
        let a = (ca >> shift) & 0xFF;
        let b = (cb >> shift) & 0xFF;
        (a as f64 + (b as f64 - a as f64) * u) as u32
    };
    (lerp_ch(16) << 16) | (lerp_ch(8) << 8) | lerp_ch(0)
}
