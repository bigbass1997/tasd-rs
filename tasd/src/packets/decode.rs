use std::io::{Read, Seek};
use std::string::FromUtf8Error;
use byteorder::ReadBytesExt;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use crate::packets::PLen;

#[derive(Debug)]
pub enum DecodeError {
    Io(std::io::Error),
    
    /// Attempted to decode a timestamp that [UtcDateTime][time::UtcDateTime::from_unix_timestamp] doesn't support.
    #[cfg(feature = "time")]
    TimeComponent(time::error::ComponentRange),
    
    /// Ran out of bytes while parsing.
    EndOfStream,
    
    /// Failed to parse bytes into a UTF-8 string.
    InvalidUtf8,
    
    /// Failed to parse an integer into an enum.
    InvalidEnum,
    
    /// Failed to parse a bool. Values larger than 1 are invalid.
    InvalidBool,
    
    /// The packet key read from the input doesn't match the target type's assigned key value.
    WrongKey,
    
    /// Returned when a length is larger than [usize::MAX].
    OversizedLength,
    
    /// Returned when a length value is missing due to the exponent being set to zero.
    /// 
    /// Arbitrarily sized length values are prefixed by a single byte (aka the "exponent" or "PEXP")
    /// which specifies how many bytes the length value is stored in.
    /// 
    /// The TASD specification requires that this exponent is never zero.
    ExponentIsZero,
    
    /// Returned when the length of a field or payload doesn't match what is expected by the TASD spec.
    /// 
    /// This usually happens when the length is expected to be a multiple of some fixed number, but
    /// instead returns a non-zero remainder.
    WrongLength,
}
impl From<std::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        if value.kind() == std::io::ErrorKind::UnexpectedEof {
            Self::EndOfStream
        } else {
            Self::Io(value)
        }
    }
}
impl From<FromUtf8Error> for DecodeError {
    fn from(_value: FromUtf8Error) -> Self {
        Self::InvalidUtf8
    }
}
impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for DecodeError {
    fn from(_value: TryFromPrimitiveError<T>) -> Self {
        Self::InvalidEnum
    }
}

#[cfg(feature = "time")]
impl From<time::error::ComponentRange> for DecodeError {
    fn from(value: time::error::ComponentRange) -> Self {
        Self::TimeComponent(value)
    }
}

macro_rules! impl_decode_prim {
    ($($t:ty)*) => ($(
        impl Decode for $t {
            fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
                paste::paste! { Ok(reader.[<read_ $t>]::<byteorder::BigEndian>()?) }
            }
        }
    )*)
}

pub trait Decode: Sized {
    /// Try to decode a single TASD packet from a `reader` ([Read] + [Seek]).
    /// 
    /// The `reader` must contain at least one valid packet, and must begin at the start of a
    /// packet.
    ///
    /// If _decoding_ fails for any reason, the reader's cursor position will be moved back to
    /// where it was when this function was first called.
    /// 
    /// However, if the reader's [Seek] implementation does not support rewinds (negative seeks),
    /// then the cursor will **not** be moved back and an [Io error][std::io::Error] will
    /// be returned instead.
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError>;
}

impl Decode for u8 {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(reader.read_u8()?)
    }
}

impl_decode_prim! { u16 i16 u32 i32 u64 i64 }

impl Decode for bool {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        // The TASD spec requires booleans to either be 0 or 1
        Ok(match reader.read_u8()? {
            0 => false,
            1 => true,
            
            _ => return Err(DecodeError::InvalidBool)
        })
    }
}

#[cfg(feature = "time")]
impl Decode for time::UtcDateTime {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(time::UtcDateTime::from_unix_timestamp(i64::decode(reader)?)?)
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut buf = [0u8; N];
        reader.read_exact(&mut buf)?;
        
        Ok(buf)
    }
}

pub(super) struct U8Vec(pub Vec<u8>);
impl Decode for U8Vec {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        let len = u8::decode(reader)? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        
        Ok(U8Vec(buf))
    }
}

pub(super) struct U8String(pub String);
impl Decode for U8String {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        let U8Vec(bytes) = U8Vec::decode(reader)?;
        
        Ok(U8String(String::from_utf8(bytes)?))
    }
}

impl Decode for PLen {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        let exp = u8::decode(reader)?;
        if exp == 0 {
            return Err(DecodeError::ExponentIsZero);
        }
        
        let mut len = 0usize;
        for _ in 0..exp {
            let Some(shifted) = len.checked_shl(8) else { return Err(DecodeError::OversizedLength) };
            len = shifted | (u8::decode(reader)? as usize);
        }
        
        Ok(PLen(len))
    }
}