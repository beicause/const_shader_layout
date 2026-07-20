#[doc(hidden)]
pub const fn write_str(buf: &mut [u8], mut pos: usize, s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        buf[pos] = bytes[i];
        pos += 1;
        i += 1;
    }
    pos
}

#[doc(hidden)]
pub const fn write_u64(buf: &mut [u8], mut pos: usize, val: u64) -> usize {
    if val == 0 {
        buf[pos] = b'0';
        return pos + 1;
    }
    let mut tmp = [0u8; 20];
    let mut i = 20;
    let mut n = val;
    while n > 0 {
        i -= 1;
        tmp[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    while i < 20 {
        buf[pos] = tmp[i];
        pos += 1;
        i += 1;
    }
    pos
}

#[doc(hidden)]
pub const fn write_usize(buf: &mut [u8], pos: usize, val: usize) -> usize {
    write_u64(buf, pos, val as u64)
}

#[doc(hidden)]
pub const fn buf_to_str<const N: usize>(buf: &[u8; N], pos: usize) -> &str {
    let len = if pos > N { N } else { pos };
    match core::str::from_utf8(buf.as_slice().split_at(len).0) {
        Ok(s) => s,
        Err(_) => "Internal error: message is not valid utf8",
    }
}
