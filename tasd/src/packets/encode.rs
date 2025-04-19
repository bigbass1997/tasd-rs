use std::io::{Error, Write};
use byteorder::WriteBytesExt;
use crate::packets::{PLen, Packet};

macro_rules! impl_encode_prim {
    ($($t:ty)*) => ($(
        impl Encode for $t {
            fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
                paste::paste! { writer.[<write_ $t>]::<byteorder::BigEndian>(*self)?; }
                Ok(size_of::<$t>())
            }
        }
    )*)
}

pub trait Encode {
    /// Encode a packet according to the TASD specification into the `writer`, returning how many
    /// bytes were written.
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error>;
}

impl Encode for u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        writer.write_u8(*self)?;
        Ok(size_of::<u8>())
    }
}

impl_encode_prim! { u16 i16 u32 i32 u64 i64 }

impl Encode for bool {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        writer.write_u8(*self as u8)?;
        Ok(size_of::<bool>())
    }
}

#[cfg(feature = "time")]
impl Encode for time::UtcDateTime {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.unix_timestamp().encode(writer)
    }
}

impl Encode for &[u8] {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write_all(self)?;
        Ok(self.len())
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write_all(self)?;
        Ok(N)
    }
}

impl Encode for &str {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write_all(self.as_bytes())?;
        Ok(self.len())
    }
}

impl Encode for Vec<u8> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.as_slice().encode(writer)
    }
}

impl Encode for Vec<u64> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let mut written = 0;
        for word in self {
            written += word.encode(writer)?;
        }
        
        Ok(written)
    }
}

impl Encode for String {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.as_bytes().encode(writer)
    }
}

impl Encode for Option<Box<Packet>> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        if let Some(p) = self {
            p.encode(writer)
        } else {
            Ok(0)
        }
    }
}

impl Encode for PLen {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        if self.0 == 0 {
            writer.write_all(&[0])?;
            return Ok(1);
        }
        
        let mut plen = Vec::with_capacity(4);
        let exp = {
            let mut tmp = self.0;
            let mut exp = 0u8;
            while tmp > 0 {
                plen.insert(0, tmp as u8);
                tmp >>= 8;
                exp += 1;
            }
            exp
        };
        
        writer.write_all(&[exp])?;
        writer.write_all(&plen)?;
        
        Ok(1 + exp as usize)
    }
}
