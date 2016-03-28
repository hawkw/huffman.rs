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

    #[inline] fn empty_buf(&mut self) -> Result<usize> {
        let byte = &[self.buf << (8 - self.n)];
        self.n = 0;
        self.buf = 0;
        self.out.write(byte)
    }

    pub fn write_bit(&mut self, bit: bool) -> Result<usize> {
        self.buf <<= 1;
        if bit { self.buf |= 1; }
        self.n += 1;
        if self.n == 8 {
            self.empty_buf()
        } else {
            Ok(0)
        }
    }

    #[inline] pub fn write_byte(&mut self, byte: u8) -> Result<usize> {
        self.write(&[byte])
    }
}

impl<'a, W: Write> Write for BitWriter<'a, W> {

    fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        if self.n == 0 {
            // if we're aligned on a byte boundary we can just write the byte
            self.out.write(bytes)
        } else {
            // otherwise, we have to write out the byte one bit at a time
            let mut written = 0;
            for byte in bytes {
                for bit in 0..8 {
                    written += try!(self.write_bit((1 << bit) & byte == 1))
                }
            }
            Ok(written)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.empty_buf()
            .and_then(|_| self.out.flush())
    }

}
