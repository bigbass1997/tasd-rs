use std::io::{Cursor, Read, Seek};
use camino::Utf8PathBuf;
use crate::packets::{DumpCreated, Encode, Decode, Packet, DecodeError};

//pub mod legacy;
pub mod packets;

pub const LATEST_VERSION: [u8; 2] = [0x00, 0x01];
pub const MAGIC_NUMBER: [u8; 4] = [0x54, 0x41, 0x53, 0x44];

#[derive(Debug)]
pub enum TasdError {
    Io(std::io::Error),
    Packet(DecodeError),
    MissingHeader,
    MagicNumberMismatch(Vec<u8>),
    UnsupportedVersion,
    MissingPath,
}
impl From<std::io::Error> for TasdError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<DecodeError> for TasdError {
    fn from(value: DecodeError) -> Self {
        Self::Packet(value)
    }
}

/// Represents a Tool Assisted Speedrun Dump (TASD) file.
#[derive(Debug, Clone, PartialEq)]
pub struct TasdFile {
    pub version: u16,
    pub keylen: u8,
    pub packets: Vec<Packet>,
    pub path: Option<Utf8PathBuf>,
}
impl Default for TasdFile {
    fn default() -> Self { Self {
        version: u16::from_be_bytes(LATEST_VERSION),
        keylen: 2,
        packets: vec![],
        path: None,
    }}
}
impl TasdFile {
    /// Creates a new [TasdFile] with a [DumpCreated] packet.
    pub fn new() -> Self {
        let mut tasd = Self::default();
        tasd.packets.push( DumpCreated::now().into() );
        
        tasd
    }
    
    /// Attempts to parse a local file into a [TasdFile].
    /// 
    /// No modifications will be made to either the local or parsed file data.
    pub fn parse_file<P: Into<Utf8PathBuf>>(path: P) -> Result<Self, TasdError> {
        let path = path.into();
        let data = std::fs::read(&path)?;
        let mut file = Self::parse_slice(&data)?;
        file.path = Some(path);
        
        Ok(file)
    }
    
    /// Attempts to parse a byte slice into a [TasdFile].
    /// 
    /// The slice must start with a valid TASD header and must end at a packet boundary.
    /// 
    /// No modifications will be made to the parsed file data.
    pub fn parse_slice(data: &[u8]) -> Result<Self, TasdError> {
        let mut reader = Cursor::new(data);
        let mut magic = vec![0u8; 4];
        reader.read_exact(&mut magic).map_err(|_| TasdError::MissingHeader)?;
        if magic != MAGIC_NUMBER {
            return Err(TasdError::MagicNumberMismatch(magic));
        }
        
        let version = u16::decode(&mut reader).map_err(|_| TasdError::MissingHeader)?;
        if ![1..=1].iter().any(|range| range.contains(&version)) {
            return Err(TasdError::UnsupportedVersion);
        }
        
        let keylen = u8::decode(&mut reader).map_err(|_| TasdError::MissingHeader)?;
        
        let mut packets = vec![];
        loop {
            match Packet::decode(&mut reader) {
                Ok(p) => packets.push(p),
                Err(DecodeError::EndOfStream) => {
                    if reader.stream_position()? as usize != data.len() {
                        return Err(DecodeError::EndOfStream.into());
                    }
                    
                    break;
                }
                Err(err) => return Err(err.into()),
            }
        }
        
        Ok(Self {
            version,
            keylen,
            packets,
            path: None,
        })
    }
    
    /// Encodes this [TasdFile] into a TASD formatted [`Vec<u8>`][Vec].
    pub fn encode(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Cursor::new(Vec::with_capacity(8));
        
        MAGIC_NUMBER.encode(&mut w)?;
        self.version.encode(&mut w)?;
        self.keylen.encode(&mut w)?;
        
        for packet in &self.packets {
            packet.encode(&mut w)?;
        }
        
        Ok(w.into_inner())
    }
    
    /// Attempts to save this file to the path specified in [`self.path`][field@TasdFile::path].
    /// 
    /// If the `path` is `None`, or any IO errors are encountered, a [TasdError] is returned, otherwise `Ok(())`.
    pub fn save(&self) -> Result<(), TasdError> {
        if let Some(path) = self.path.as_ref() {
            std::fs::write(path, self.encode()?)?;
        } else {
            return Err(TasdError::MissingPath)
        }
        
        Ok(())
    }
}
