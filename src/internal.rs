#[doc(hidden)]
pub struct MsgBuf<const N: usize> {
    buf: [u8; N],
    pos: usize,
}

impl<const N: usize> MsgBuf<N> {
    #[expect(clippy::new_without_default, reason = "Default::default() is not const fn, unusable in const blocks")]
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            pos: 0,
        }
    }

    pub const fn write_str(&mut self, s: &str) -> &mut Self {
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            self.buf[self.pos] = bytes[i];
            self.pos += 1;
            i += 1;
        }
        self
    }

    pub const fn write_u64(&mut self, val: u64) -> &mut Self {
        if val == 0 {
            self.buf[self.pos] = b'0';
            self.pos += 1;
            return self;
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
            self.buf[self.pos] = tmp[i];
            self.pos += 1;
            i += 1;
        }
        self
    }

    pub const fn write_usize(&mut self, val: usize) -> &mut Self {
        self.write_u64(val as u64)
    }

    pub const fn as_str(&self) -> &str {
        let len = if self.pos > N { N } else { self.pos };
        match core::str::from_utf8(self.buf.as_slice().split_at(len).0) {
            Ok(s) => s,
            Err(_) => "Internal error: message is not valid utf8",
        }
    }
}
