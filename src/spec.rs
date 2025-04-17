use std::time::{SystemTime, UNIX_EPOCH};
use camino::Utf8PathBuf;
use crate::spec::packets::{DumpCreated, Encode, Packet, PacketError};
use crate::spec::reader::Reader;
use crate::spec::writer::Writer;

pub mod legacy;
pub mod packets;
pub mod reader;
pub mod writer;

pub const LATEST_VERSION: [u8; 2] = [0x00, 0x01];
pub const MAGIC_NUMBER: [u8; 4] = [0x54, 0x41, 0x53, 0x44];

#[derive(Debug)]
pub enum TasdError {
    Io(std::io::Error),
    Packet(PacketError),
    MissingHeader,
    MagicNumberMismatch(Vec<u8>),
    MissingPath,
}
impl From<std::io::Error> for TasdError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<PacketError> for TasdError {
    fn from(value: PacketError) -> Self {
        Self::Packet(value)
    }
}


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
        path: None
    }}
}
impl TasdFile {
    pub fn new() -> Self {
        let mut tasd = Self::default();
        tasd.packets.push(
            DumpCreated {
                epoch: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time has gone backwards?").as_secs() as i64
            }.into()
        );
        
        tasd
    }
    
    pub fn parse_file<P: Into<Utf8PathBuf>>(path: P) -> Result<Self, TasdError> {
        let path = path.into();
        let data = std::fs::read(&path)?;
        let mut file = Self::parse_slice(&data)?;
        file.path = Some(path);
        
        Ok(file)
    }
    
    pub fn parse_slice(data: &[u8]) -> Result<Self, TasdError> {
        let mut r = Reader::new(&data);
        if r.remaining() < 7 {
            return Err(TasdError::MissingHeader);
        }
        let magic = r.read_len(4);
        if magic != MAGIC_NUMBER {
            return Err(TasdError::MagicNumberMismatch(magic.to_vec()));
        }
        
        let mut file = Self {
            version: r.read_u16(),
            keylen: r.read_u8(),
            packets: vec![],
            path: None,
        };
        
        while r.remaining() > 0 {
            use PacketError::*;
            match Packet::with_reader(&mut r, file.keylen) {
                Ok(packet) => file.packets.push(packet),
                Err(err) => match err {
                    MissingKey | MismatchedKey | MissingPayloadLength | UnsupportedExponent(_) => return Err(err.into()),
                    InvalidPayload { key, payload } => println!("InvalidPayload! Skipping. ({key:02X?}, {payload:02X?}"),
                }
            }
        }
        
        Ok(file)
    }
    
    /// Encodes data in this [TasdFile] into a TASD formatted Vec of bytes.
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_slice(&MAGIC_NUMBER);
        w.write_slice(&LATEST_VERSION);
        w.write_u8(self.keylen);
        
        for packet in &self.packets {
            w.write_slice(&packet.encode(self.keylen));
        }
        
        w.to_vec()
    }
    
    /// Attempts to save this file to the path specified in [`self.path`][field@TasdFile::path].
    /// 
    /// If the path is `None`, or any IO errors are encountered, an `Err` is returned, otherwise `Ok(())`.
    pub fn save(&self) -> Result<(), TasdError> {
        if let Some(path) = self.path.as_ref() {
            std::fs::write(path, self.encode()).map_err(|err| err.into())
        } else {
            Err(TasdError::MissingPath)
        }
    }
}
