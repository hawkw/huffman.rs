//! Utility for writing to a  stream of bits

use std::io::{Write, Result};

pub struct BitWriter<'a, W>
where W: Write
    , W: 'a { out: &'a mut W
            , buf: u8
            , n: u8
            }

impl<'a, W: Write> BitWriter<'a, W> {
    pub fn new(w: &'a mut W) -> BitWriter<'a, W> {
        BitWriter { out: w, buf: 0, n: 0 }
    }

    pub fn flush(&mut self) -> Result<()> {
        self.empty_buf();
        self.out.flush()
    }

    #[inline] fn empty_buf(&mut self) {
        let byte = &[self.buf << (8 - self.n)];
        self.n = 0;
        self.buf = 0;
        self.out.write(byte)
            .expect("Could not flush buffer!");
    }

    pub fn write_bit(&mut self, bit: bool) {
        self.buf <<= 1;
        if bit { self.buf |= 1; }
        self.n += 1;
        if self.n == 8 {
            self.empty_buf()
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.n == 0 {
            // if we're aligned on a byte boundary we can just write the byte
            self.out.write(&[byte])
                .expect("Could not write byte!");
        } else {
            // otherwise, we have to write out the byte one bit at a time
            for bit in 0..8 {
                self.write_bit((1 << bit) & byte == 1);
            }
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        if self.n == 0 {
            // if we're aligned on a byte boundary we can just write the byte
            self.out.write(bytes)
                .expect("Could not write bytes!");
        } else {
            // otherwise, we have to write out the byte one bit at a time
            for byte in bytes {
                for bit in 0..8 {
                    self.write_bit((1 << bit) & byte == 1);
                }
            }
        }
    }
}
