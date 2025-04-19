use std::io::{Error, Seek};
use std::io::Read;
use std::io::Write;
use derive_more::From;
use derive_more::with_trait::{IsVariant, TryUnwrap, Unwrap};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_macros::{Packet, Wrapper};

mod decode;
mod encode;

pub use decode::*;
pub use encode::*;

#[derive(Debug, Clone, PartialEq, From, IsVariant, TryUnwrap, Unwrap, Wrapper)]
pub enum Packet {
    ConsoleType(ConsoleType),
    ConsoleRegion(ConsoleRegion),
    GameTitle(GameTitle),
    RomName(RomName),
    Attribution(Attribution),
    Category(Category),
    EmulatorName(EmulatorName),
    EmulatorVersion(EmulatorVersion),
    EmulatorCore(EmulatorCore),
    TasLastModified(TasLastModified),
    DumpCreated(DumpCreated),
    DumpLastModified(DumpLastModified),
    TotalFrames(TotalFrames),
    Rerecords(Rerecords),
    SourceLink(SourceLink),
    BlankFrames(BlankFrames),
    Verified(Verified),
    MemoryInit(MemoryInit),
    GameIdentifier(GameIdentifier),
    MovieLicense(MovieLicense),
    MovieFile(MovieFile),
    PortController(PortController),
    PortOverread(PortOverread),
    NesLatchFilter(NesLatchFilter),
    NesClockFilter(NesClockFilter),
    NesGameGenieCode(NesGameGenieCode),
    SnesLatchFilter(SnesLatchFilter),
    SnesClockFilter(SnesClockFilter),
    SnesGameGenieCode(SnesGameGenieCode),
    SnesLatchTrain(SnesLatchTrain),
    GenesisGameGenieCode(GenesisGameGenieCode),
    InputChunk(InputChunk),
    InputMoment(InputMoment),
    Transition(Transition),
    LagFrameChunk(LagFrameChunk),
    MovieTransition(MovieTransition),
    Comment(Comment),
    Experimental(Experimental),
    Unspecified(Unspecified),
    
    #[unsupported]
    Unsupported(Unsupported),
}

struct PLen(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Unsupported {
    key: Vec<u8>,
    data: Vec<u8>,
}
impl Encode for Unsupported {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let mut written = 0;
        written += self.key.encode(writer)?;
        written += PLen(self.data.len()).encode(writer)?;
        written += self.data.encode(writer)?;
        
        Ok(written)
    }
}
impl Decode for Unsupported {
    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut key = vec![0u8; 2];
        reader.read_exact(&mut key)?;
        let plen = PLen::decode(reader)?.0;
        let mut data = vec![0u8; plen];
        reader.read_exact(&mut data)?;
        
        Ok(Self { key, data })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Console {
    Nes = 0x01,
    Snes = 0x02,
    N64 = 0x03,
    Gc = 0x04,
    Gb = 0x05,
    Gbc = 0x06,
    Gba = 0x07,
    Genesis = 0x08,
    A2600 = 0x09,
    Custom = 0xFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x01]"]
pub struct ConsoleType {
    #[u8_enum]
    pub console: Console,
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive, Packet)]
#[key = "[0x00, 0x02]"]
#[repr(u8)]
pub enum ConsoleRegion {
    Ntsc = 0x01,
    Pal = 0x02,
    Other = 0xFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x03]"]
pub struct GameTitle {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x04]"]
pub struct RomName {
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum AttributionKind {
    Author = 0x01,
    Verifier = 0x02,
    TasdFileCreator = 0x03,
    TasdFileEditor = 0x04,
    Other = 0xFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x05]"]
pub struct Attribution {
    #[u8_enum]
    pub kind: AttributionKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x06]"]
pub struct Category {
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x07]"]
pub struct EmulatorName {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x08]"]
pub struct EmulatorVersion {
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x09]"]
pub struct EmulatorCore {
    pub core: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0A]"]
pub struct TasLastModified {
    #[cfg(feature = "time")]
    pub timestamp: time::UtcDateTime,
    
    #[cfg(not(feature = "time"))]
    pub timestamp: i64,
}
impl TasLastModified {
    pub fn now() -> Self {
        Self {
            #[cfg(feature = "time")]
            timestamp: time::UtcDateTime::now(),
            
            #[cfg(not(feature = "time"))]
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time has gone backwards?").as_secs() as i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0B]"]
pub struct DumpCreated {
    #[cfg(feature = "time")]
    pub timestamp: time::UtcDateTime,
    
    #[cfg(not(feature = "time"))]
    pub timestamp: i64,
}
impl DumpCreated {
    pub fn now() -> Self {
        Self {
            #[cfg(feature = "time")]
            timestamp: time::UtcDateTime::now(),
            
            #[cfg(not(feature = "time"))]
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time has gone backwards?").as_secs() as i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0C]"]
pub struct DumpLastModified {
    #[cfg(feature = "time")]
    pub timestamp: time::UtcDateTime,
    
    #[cfg(not(feature = "time"))]
    pub timestamp: i64,
}
impl DumpLastModified {
    pub fn now() -> Self {
        Self {
            #[cfg(feature = "time")]
            timestamp: time::UtcDateTime::now(),
            
            #[cfg(not(feature = "time"))]
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Time has gone backwards?").as_secs() as i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0D]"]
pub struct TotalFrames {
    pub frames: u32,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0E]"]
pub struct Rerecords {
    pub rerecords: u32,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x0F]"]
pub struct SourceLink {
    pub link: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x10]"]
pub struct BlankFrames {
    pub frames: i16,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x11]"]
pub struct Verified {
    pub verified: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum InitKind {
    NoInitialization = 0x01,
    AllZeros = 0x02,
    AllOnes = 0x03,
    /// `[00 00 00 00 FF FF FF FF]` in a repeating pattern
    Repeating4Zeros4FF = 0x04,
    Random = 0x05,
    Custom = 0xFF,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum InitDevice {
    NesCpuRam = 0x0101,
    NesCartridgeSaveData = 0x0102,
    SnesCpuRam = 0x0201,
    SnesCartridgeSaveData = 0x0202,
    GbCpuRam = 0x0501,
    GbCartridgeSaveData = 0x0502,
    GbcCpuRam = 0x0601,
    GbcCartridgeSaveData = 0x0602,
    GbaCpuRam = 0x0701,
    GbaCartridgeSaveData = 0x0702,
    GenesisCpuRam = 0x0801,
    GenesisCartridgeSaveData = 0x0802,
    A2600CpuRam = 0x0901,
    A2600CartridgeSaveData = 0x0902,
    CustomDevice = 0xFFFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x12]"]
pub struct MemoryInit {
    #[u8_enum]
    pub data_type: InitKind,
    #[u16_enum]
    pub device: InitDevice,
    pub required: bool,
    #[u8_string]
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum IdKind {
    Md5Hash = 0x01,
    Sha1Hash = 0x02,
    Sha224Hash = 0x03,
    Sha256Hash = 0x04,
    Sha384Hash = 0x05,
    Sha512Hash = 0x06,
    Sha512_224Hash = 0x07,
    Sha512_256Hash = 0x08,
    Sha3_224Hash = 0x09,
    Sha3_256Hash = 0x0A,
    Sha3_384Hash = 0x0B,
    Sha3_512Hash = 0x0C,
    Shake128Hash = 0x0D,
    Shake256Hash = 0x0E,
    Other = 0xFF,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum IdEncoding {
    RawBinary = 0x01,
    /// Case Insensitive
    Base16 = 0x02,
    /// Case Insensitive
    Base32 = 0x03,
    Base64 = 0x04,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x13]"]
pub struct GameIdentifier {
    #[u8_enum]
    pub kind: IdKind,
    #[u8_enum]
    pub encoding: IdEncoding,
    #[u8_string]
    pub name: String,
    pub identifier: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x14]"]
pub struct MovieLicense {
    pub license: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0x15]"]
pub struct MovieFile {
    #[u8_string]
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum PortKind {
    NesStandardController = 0x0101,
    NesFourScore = 0x0102,
    /// Reserved
    NesZapper = 0x0103,
    /// Reserved
    NesPowerPad = 0x0104,
    /// Reserved
    FamicomFamilyBasicKeyboard = 0x0105,
    SnesStandardController = 0x0201,
    SnesSuperMultitap = 0x0202,
    SnesMouse = 0x0203,
    /// Reserved
    SnesSuperscope = 0x0204,
    N64StandardController = 0x0301,
    N64StandardControllerWithRumblePak = 0x0302,
    N64StandardControllerWithControllerPak = 0x0303,
    N64StandardControllerWithTransferPak = 0x0304,
    N64Mouse = 0x0305,
    /// Reserved
    N64VoiceRecognitionUnit = 0x0306,
    /// Reserved
    N64RandNetKeyboard = 0x0307,
    N64DenshaDeGo = 0x0308,
    GcStandardController = 0x0401,
    /// Reserved
    GcKeyboard = 0x0402,
    GbGamepad = 0x0501,
    GbcGamepad = 0x0601,
    GbaGamepad = 0x0701,
    Genesis3Button = 0x0801,
    Genesis6Button = 0x0802,
    A2600Joystick = 0x0901,
    /// Reserved
    A2600Paddle = 0x0902,
    A2600KeyboardController = 0x0903,
    Other = 0xFFFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0xF0]"]
pub struct PortController {
    pub port: u8,
    #[u16_enum]
    pub kind: PortKind,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x00, 0xF1]"]
pub struct PortOverread {
    pub port: u8,
    pub high: bool,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x01, 0x01]"]
pub struct NesLatchFilter {
    pub time: u16,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x01, 0x02]"]
pub struct NesClockFilter {
    pub time: u8,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x01, 0x04]"]
pub struct NesGameGenieCode {
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x02, 0x01]"]
pub struct SnesLatchFilter {
    pub time: u16,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x02, 0x02]"]
pub struct SnesClockFilter {
    pub time: u8,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x02, 0x04]"]
pub struct SnesGameGenieCode {
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x02, 0x05]"]
pub struct SnesLatchTrain {
    pub latch_trains: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0x08, 0x04]"]
pub struct GenesisGameGenieCode {
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFE, 0x01]"]
pub struct InputChunk {
    pub port: u8,
    pub inputs: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MomentIndexKind {
    Frame = 0x01,
    CycleCount = 0x02,
    Milliseconds = 0x03,
    Microseconds = 0x04,
    Nanoseconds = 0x05,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFE, 0x02]"]
pub struct InputMoment {
    pub port: u8,
    pub hold: bool,
    #[u8_enum]
    pub index_type: MomentIndexKind,
    pub index: u64,
    pub inputs: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TransitionIndexKind {
    Frame = 0x01,
    CycleCount = 0x02,
    Milliseconds = 0x03,
    Microseconds = 0x04,
    Nanoseconds = 0x05,
    InputChunkByteIndex = 0x06,
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TransitionKind {
    SoftReset = 0x01,
    PowerReset = 0x02,
    RestartTasdFile = 0x03,
    PacketDerived = 0xFF,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFE, 0x03]"]
pub struct Transition {
    pub port: u8,
    #[u8_enum]
    pub index_type: TransitionIndexKind,
    pub index: u64,
    #[u8_enum]
    pub transition_type: TransitionKind,
    pub inner_packet: Option<Box<Packet>>,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFE, 0x04]"]
pub struct LagFrameChunk {
    pub movie_frame: u32,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFE, 0x05]"]
pub struct MovieTransition {
    pub movie_frame: u32,
    #[u8_enum]
    pub transition_type: TransitionKind,
    pub inner_packet: Option<Box<Packet>>,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFF, 0x01]"]
pub struct Comment {
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFF, 0xFE]"]
pub struct Experimental {
    pub experimental: bool,
}

#[derive(Debug, Clone, PartialEq, Packet)]
#[key = "[0xFF, 0xFF]"]
pub struct Unspecified {
    pub data: Vec<u8>,
}















#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    
    #[test]
    fn console_type() {
        let p = ConsoleType {
            console: Console::Custom,
            name: "0123456789".into(),
        };
        
        let mut buf = vec![];
        assert_eq!(p.encode(&mut buf).unwrap(), 15);
        
        println!("{buf:02X?}");
        
        let de_p = ConsoleType::decode(&mut Cursor::new(buf)).unwrap();
        assert_eq!(p, de_p);
        println!("{de_p:#?}");
    }
    
    #[test]
    fn wrapper() {
        let p: Packet = ConsoleType {
            console: Console::Custom,
            name: "0123456789".into(),
        }.into();
        
        let mut buf = vec![];
        assert_eq!(p.encode(&mut buf).unwrap(), 15);
        
        println!("{buf:02X?}");
        
        let de_p = Packet::decode(&mut Cursor::new(buf)).unwrap();
        assert_eq!(p, de_p);
        println!("{de_p:#?}");
    }
    
    #[test]
    fn unsupported() {
        let p: Packet = Unsupported {
            key: vec![0xA5, 0x5A],
            data: b"0123456789".to_vec(),
        }.into();
        
        let mut buf = vec![];
        assert_eq!(p.encode(&mut buf).unwrap(), 14);
        
        println!("{buf:02X?}");
        
        let de_p = Packet::decode(&mut Cursor::new(buf)).unwrap();
        assert_eq!(p, de_p);
        println!("{de_p:02X?}");
    }
}