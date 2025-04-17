use std::fmt::Debug;
use crate::spec::reader::Reader;
use crate::spec::writer::Writer;

macro_rules! impl_from_packet {
    ($($name:ident)*) => ($(
        impl From<$name> for Packet {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    )*)
}

pub const KEY_CONSOLE_TYPE: &[u8] =         &[0x00, 0x01];
pub const KEY_CONSOLE_REGION: &[u8] =       &[0x00, 0x02];
pub const KEY_GAME_TITLE: &[u8] =           &[0x00, 0x03];
pub const KEY_ROM_NAME: &[u8] =             &[0x00, 0x04];
pub const KEY_ATTRIBUTION: &[u8] =          &[0x00, 0x05];
pub const KEY_CATEGORY: &[u8] =             &[0x00, 0x06];
pub const KEY_EMULATOR_NAME: &[u8] =        &[0x00, 0x07];
pub const KEY_EMULATOR_VERSION: &[u8] =     &[0x00, 0x08];
pub const KEY_EMULATOR_CORE: &[u8] =        &[0x00, 0x09];
pub const KEY_TAS_LAST_MODIFIED: &[u8] =    &[0x00, 0x0A];
pub const KEY_DUMP_CREATED: &[u8] =         &[0x00, 0x0B];
pub const KEY_DUMP_LAST_MODIFIED: &[u8] =   &[0x00, 0x0C];
pub const KEY_TOTAL_FRAMES: &[u8] =         &[0x00, 0x0D];
pub const KEY_RERECORDS: &[u8] =            &[0x00, 0x0E];
pub const KEY_SOURCE_LINK: &[u8] =          &[0x00, 0x0F];
pub const KEY_BLANK_FRAMES: &[u8] =         &[0x00, 0x10];
pub const KEY_VERIFIED: &[u8] =             &[0x00, 0x11];
pub const KEY_MEMORY_INIT: &[u8] =          &[0x00, 0x12];
pub const KEY_GAME_IDENTIFIER: &[u8] =      &[0x00, 0x13];
pub const KEY_MOVIE_LICENSE: &[u8] =        &[0x00, 0x14];
pub const KEY_MOVIE_FILE: &[u8] =           &[0x00, 0x15];

pub const KEY_PORT_CONTROLLER: &[u8] =      &[0x00, 0xF0];
pub const KEY_PORT_OVERREAD: &[u8] =         &[0x00, 0xF1];

pub const KEY_NES_LATCH_FILTER: &[u8] =     &[0x01, 0x01];
pub const KEY_NES_CLOCK_FILTER: &[u8] =     &[0x01, 0x02];

pub const KEY_NES_GAME_GENIE_CODE: &[u8] =  &[0x01, 0x04];

pub const KEY_SNES_LATCH_FILTER: &[u8] =    &[0x02, 0x01];
pub const KEY_SNES_CLOCK_FILTER: &[u8] =    &[0x02, 0x02];

pub const KEY_SNES_GAME_GENIE_CODE: &[u8] = &[0x02, 0x04];
pub const KEY_SNES_LATCH_TRAIN: &[u8] =     &[0x02, 0x05];

pub const KEY_GENESIS_GAME_GENIE_CODE: &[u8] = &[0x08, 0x04];

pub const KEY_INPUT_CHUNK: &[u8] =          &[0xFE, 0x01];
pub const KEY_INPUT_MOMENT: &[u8] =         &[0xFE, 0x02];
pub const KEY_TRANSITION: &[u8] =           &[0xFE, 0x03];
pub const KEY_LAG_FRAME_CHUNK: &[u8] =      &[0xFE, 0x04];
pub const KEY_MOVIE_TRANSITION: &[u8] =     &[0xFE, 0x05];

pub const KEY_COMMENT: &[u8] =              &[0xFF, 0x01];
pub const KEY_EXPERIMENTAL: &[u8] =         &[0xFF, 0xFE];
pub const KEY_UNSPECIFIED: &[u8] =          &[0xFF, 0xFF];

#[derive(Debug)]
pub enum PacketError {
    MissingKey,
    MismatchedKey,
    MissingPayloadLength,
    UnsupportedExponent(u8),
    InvalidPayload {
        key: Vec<u8>,
        payload: Vec<u8>,
    },
}
impl PacketError {
    pub(crate) fn invalid(key: &[u8], payload: Reader) -> Self {
        Self::InvalidPayload {
            key: key.to_vec(),
            payload: payload.to_vec(),
        }
    }
}


pub trait Decode: Sized + Debug + Clone + PartialEq {
    fn decode(key: &[u8], payload: Reader) -> Result<Self, PacketError>;
    
    fn kind(&self) -> PacketKind;
    fn name(&self) -> String {
        self.kind().to_string()
    }
}

pub trait Encode: Debug + Clone + PartialEq {
    fn encode(&self, keylen: u8) -> Vec<u8>;
    
    fn key(&self) -> Vec<u8>;
}


#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
    Unsupported(Unsupported),
}
impl Packet {
    pub fn with_reader(r: &mut Reader, keylen: u8) -> Result<Packet, PacketError> {
        if r.remaining() < keylen as usize {
            return Err(PacketError::MissingKey);
        }
        let key = r.read_len(keylen as usize).to_vec();
        
        if r.remaining() < 1 {
            return Err(PacketError::MissingPayloadLength);
        }
        let exp = r.read_u8() as usize;
        
        if r.remaining() < exp {
            return Err(PacketError::MissingPayloadLength);
        }
        if exp > 8 {
            return Err(PacketError::UnsupportedExponent(exp as u8));
        }
        
        let mut plen = [0u8; 8];
        for i in (0..exp).rev() {
            plen[plen.len() - i - 1] = r.read_u8();
        }
        let plen = u64::from_be_bytes(plen);
        
        let payload = r.read_len(plen as usize);
        let payload = Reader::new(&payload);
        
        let key = key.as_slice();
        //println!("decoding key: {key:02X?}");
        Ok(match key {
            KEY_CONSOLE_TYPE => Packet::ConsoleType(ConsoleType::decode(key, payload)?),
            KEY_CONSOLE_REGION => Packet::ConsoleRegion(ConsoleRegion::decode(key, payload)?),
            KEY_GAME_TITLE => Packet::GameTitle(GameTitle::decode(key, payload)?),
            KEY_ROM_NAME => Packet::RomName(RomName::decode(key, payload)?),
            KEY_ATTRIBUTION => Packet::Attribution(Attribution::decode(key, payload)?),
            KEY_CATEGORY => Packet::Category(Category::decode(key, payload)?),
            KEY_EMULATOR_NAME => Packet::EmulatorName(EmulatorName::decode(key, payload)?),
            KEY_EMULATOR_VERSION => Packet::EmulatorVersion(EmulatorVersion::decode(key, payload)?),
            KEY_EMULATOR_CORE => Packet::EmulatorCore(EmulatorCore::decode(key, payload)?),
            KEY_TAS_LAST_MODIFIED => Packet::TasLastModified(TasLastModified::decode(key, payload)?),
            KEY_DUMP_CREATED => Packet::DumpCreated(DumpCreated::decode(key, payload)?),
            KEY_DUMP_LAST_MODIFIED => Packet::DumpLastModified(DumpLastModified::decode(key, payload)?),
            KEY_TOTAL_FRAMES => Packet::TotalFrames(TotalFrames::decode(key, payload)?),
            KEY_RERECORDS => Packet::Rerecords(Rerecords::decode(key, payload)?),
            KEY_SOURCE_LINK => Packet::SourceLink(SourceLink::decode(key, payload)?),
            KEY_BLANK_FRAMES => Packet::BlankFrames(BlankFrames::decode(key, payload)?),
            KEY_VERIFIED => Packet::Verified(Verified::decode(key, payload)?),
            KEY_MEMORY_INIT => Packet::MemoryInit(MemoryInit::decode(key, payload)?),
            KEY_GAME_IDENTIFIER => Packet::GameIdentifier(GameIdentifier::decode(key, payload)?),
            KEY_MOVIE_LICENSE => Packet::MovieLicense(MovieLicense::decode(key, payload)?),
            KEY_MOVIE_FILE => Packet::MovieFile(MovieFile::decode(key, payload)?),
            KEY_PORT_CONTROLLER => Packet::PortController(PortController::decode(key, payload)?),
            KEY_PORT_OVERREAD => Packet::PortOverread(PortOverread::decode(key, payload)?),
            KEY_NES_LATCH_FILTER => Packet::NesLatchFilter(NesLatchFilter::decode(key, payload)?),
            KEY_NES_CLOCK_FILTER => Packet::NesClockFilter(NesClockFilter::decode(key, payload)?),
            KEY_NES_GAME_GENIE_CODE => Packet::NesGameGenieCode(NesGameGenieCode::decode(key, payload)?),
            KEY_SNES_LATCH_FILTER => Packet::SnesLatchFilter(SnesLatchFilter::decode(key, payload)?),
            KEY_SNES_CLOCK_FILTER => Packet::SnesClockFilter(SnesClockFilter::decode(key, payload)?),
            KEY_SNES_GAME_GENIE_CODE => Packet::SnesGameGenieCode(SnesGameGenieCode::decode(key, payload)?),
            KEY_SNES_LATCH_TRAIN => Packet::SnesLatchTrain(SnesLatchTrain::decode(key, payload)?),
            KEY_GENESIS_GAME_GENIE_CODE => Packet::GenesisGameGenieCode(GenesisGameGenieCode::decode(key, payload)?),
            KEY_INPUT_CHUNK => Packet::InputChunk(InputChunk::decode(key, payload)?),
            KEY_INPUT_MOMENT => Packet::InputMoment(InputMoment::decode(key, payload)?),
            KEY_TRANSITION => Packet::Transition(Transition::decode(key, payload)?),
            KEY_LAG_FRAME_CHUNK => Packet::LagFrameChunk(LagFrameChunk::decode(key, payload)?),
            KEY_MOVIE_TRANSITION => Packet::MovieTransition(MovieTransition::decode(key, payload)?),
            KEY_COMMENT => Packet::Comment(Comment::decode(key, payload)?),
            KEY_EXPERIMENTAL => Packet::Experimental(Experimental::decode(key, payload)?),
            KEY_UNSPECIFIED => Packet::Unspecified(Unspecified::decode(key, payload)?),
            
            _ => Packet::Unsupported(Unsupported::decode(key, payload)?)
        })
    }
    
    pub fn kind(&self) -> PacketKind {
        match self {
            Self::ConsoleType(packet) => packet.kind(),
            Self::ConsoleRegion(packet) => packet.kind(),
            Self::GameTitle(packet) => packet.kind(),
            Self::RomName(packet) => packet.kind(),
            Self::Attribution(packet) => packet.kind(),
            Self::Category(packet) => packet.kind(),
            Self::EmulatorName(packet) => packet.kind(),
            Self::EmulatorVersion(packet) => packet.kind(),
            Self::EmulatorCore(packet) => packet.kind(),
            Self::TasLastModified(packet) => packet.kind(),
            Self::DumpCreated(packet) => packet.kind(),
            Self::DumpLastModified(packet) => packet.kind(),
            Self::TotalFrames(packet) => packet.kind(),
            Self::Rerecords(packet) => packet.kind(),
            Self::SourceLink(packet) => packet.kind(),
            Self::BlankFrames(packet) => packet.kind(),
            Self::Verified(packet) => packet.kind(),
            Self::MemoryInit(packet) => packet.kind(),
            Self::GameIdentifier(packet) => packet.kind(),
            Self::MovieLicense(packet) => packet.kind(),
            Self::MovieFile(packet) => packet.kind(),
            Self::PortController(packet) => packet.kind(),
            Self::PortOverread(packet) => packet.kind(),
            Self::NesLatchFilter(packet) => packet.kind(),
            Self::NesClockFilter(packet) => packet.kind(),
            Self::NesGameGenieCode(packet) => packet.kind(),
            Self::SnesLatchFilter(packet) => packet.kind(),
            Self::SnesClockFilter(packet) => packet.kind(),
            Self::SnesGameGenieCode(packet) => packet.kind(),
            Self::SnesLatchTrain(packet) => packet.kind(),
            Self::GenesisGameGenieCode(packet) => packet.kind(),
            Self::InputChunk(packet) => packet.kind(),
            Self::InputMoment(packet) => packet.kind(),
            Self::Transition(packet) => packet.kind(),
            Self::LagFrameChunk(packet) => packet.kind(),
            Self::MovieTransition(packet) => packet.kind(),
            Self::Comment(packet) => packet.kind(),
            Self::Experimental(packet) => packet.kind(),
            Self::Unspecified(packet) => packet.kind(),
            Self::Unsupported(packet) => packet.kind(),
        }
    }
}
impl Encode for Packet {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        match self {
            Self::ConsoleType(packet) => packet.encode(keylen),
            Self::ConsoleRegion(packet) => packet.encode(keylen),
            Self::GameTitle(packet) => packet.encode(keylen),
            Self::RomName(packet) => packet.encode(keylen),
            Self::Attribution(packet) => packet.encode(keylen),
            Self::Category(packet) => packet.encode(keylen),
            Self::EmulatorName(packet) => packet.encode(keylen),
            Self::EmulatorVersion(packet) => packet.encode(keylen),
            Self::EmulatorCore(packet) => packet.encode(keylen),
            Self::TasLastModified(packet) => packet.encode(keylen),
            Self::DumpCreated(packet) => packet.encode(keylen),
            Self::DumpLastModified(packet) => packet.encode(keylen),
            Self::TotalFrames(packet) => packet.encode(keylen),
            Self::Rerecords(packet) => packet.encode(keylen),
            Self::SourceLink(packet) => packet.encode(keylen),
            Self::BlankFrames(packet) => packet.encode(keylen),
            Self::Verified(packet) => packet.encode(keylen),
            Self::MemoryInit(packet) => packet.encode(keylen),
            Self::GameIdentifier(packet) => packet.encode(keylen),
            Self::MovieLicense(packet) => packet.encode(keylen),
            Self::MovieFile(packet) => packet.encode(keylen),
            Self::PortController(packet) => packet.encode(keylen),
            Self::PortOverread(packet) => packet.encode(keylen),
            Self::NesLatchFilter(packet) => packet.encode(keylen),
            Self::NesClockFilter(packet) => packet.encode(keylen),
            Self::NesGameGenieCode(packet) => packet.encode(keylen),
            Self::SnesLatchFilter(packet) => packet.encode(keylen),
            Self::SnesClockFilter(packet) => packet.encode(keylen),
            Self::SnesGameGenieCode(packet) => packet.encode(keylen),
            Self::SnesLatchTrain(packet) => packet.encode(keylen),
            Self::GenesisGameGenieCode(packet) => packet.encode(keylen),
            Self::InputChunk(packet) => packet.encode(keylen),
            Self::InputMoment(packet) => packet.encode(keylen),
            Self::Transition(packet) => packet.encode(keylen),
            Self::LagFrameChunk(packet) => packet.encode(keylen),
            Self::MovieTransition(packet) => packet.encode(keylen),
            Self::Comment(packet) => packet.encode(keylen),
            Self::Experimental(packet) => packet.encode(keylen),
            Self::Unspecified(packet) => packet.encode(keylen),
            Self::Unsupported(packet) => packet.encode(keylen),
        }
    }

    fn key(&self) -> Vec<u8> {
        match self {
            Self::ConsoleType(packet) => packet.key(),
            Self::ConsoleRegion(packet) => packet.key(),
            Self::GameTitle(packet) => packet.key(),
            Self::RomName(packet) => packet.key(),
            Self::Attribution(packet) => packet.key(),
            Self::Category(packet) => packet.key(),
            Self::EmulatorName(packet) => packet.key(),
            Self::EmulatorVersion(packet) => packet.key(),
            Self::EmulatorCore(packet) => packet.key(),
            Self::TasLastModified(packet) => packet.key(),
            Self::DumpCreated(packet) => packet.key(),
            Self::DumpLastModified(packet) => packet.key(),
            Self::TotalFrames(packet) => packet.key(),
            Self::Rerecords(packet) => packet.key(),
            Self::SourceLink(packet) => packet.key(),
            Self::BlankFrames(packet) => packet.key(),
            Self::Verified(packet) => packet.key(),
            Self::MemoryInit(packet) => packet.key(),
            Self::GameIdentifier(packet) => packet.key(),
            Self::MovieLicense(packet) => packet.key(),
            Self::MovieFile(packet) => packet.key(),
            Self::PortController(packet) => packet.key(),
            Self::PortOverread(packet) => packet.key(),
            Self::NesLatchFilter(packet) => packet.key(),
            Self::NesClockFilter(packet) => packet.key(),
            Self::NesGameGenieCode(packet) => packet.key(),
            Self::SnesLatchFilter(packet) => packet.key(),
            Self::SnesClockFilter(packet) => packet.key(),
            Self::SnesGameGenieCode(packet) => packet.key(),
            Self::SnesLatchTrain(packet) => packet.key(),
            Self::GenesisGameGenieCode(packet) => packet.key(),
            Self::InputChunk(packet) => packet.key(),
            Self::InputMoment(packet) => packet.key(),
            Self::Transition(packet) => packet.key(),
            Self::LagFrameChunk(packet) => packet.key(),
            Self::MovieTransition(packet) => packet.key(),
            Self::Comment(packet) => packet.key(),
            Self::Experimental(packet) => packet.key(),
            Self::Unspecified(packet) => packet.key(),
            Self::Unsupported(packet) => packet.key(),
        }
    }
}
impl_from_packet!(
    ConsoleType
    ConsoleRegion
    GameTitle
    RomName
    Attribution
    Category
    EmulatorName
    EmulatorVersion
    EmulatorCore
    TasLastModified
    DumpCreated
    DumpLastModified
    TotalFrames
    Rerecords
    SourceLink
    BlankFrames
    Verified
    MemoryInit
    GameIdentifier
    MovieLicense
    MovieFile
    PortController
    PortOverread
    NesLatchFilter
    NesClockFilter
    NesGameGenieCode
    SnesLatchFilter
    SnesClockFilter
    SnesGameGenieCode
    SnesLatchTrain
    GenesisGameGenieCode
    InputChunk
    InputMoment
    Transition
    LagFrameChunk
    MovieTransition
    Comment
    Experimental
    Unspecified
    Unsupported
);

#[derive(Debug, Copy, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PacketKind {
    ConsoleType,
    ConsoleRegion,
    GameTitle,
    RomName,
    Attribution,
    Category,
    EmulatorName,
    EmulatorVersion,
    EmulatorCore,
    TasLastModified,
    DumpCreated,
    DumpLastModified,
    TotalFrames,
    Rerecords,
    SourceLink,
    BlankFrames,
    Verified,
    MemoryInit,
    GameIdentifier,
    MovieLicense,
    MovieFile,
    PortController,
    PortOverread,
    NesLatchFilter,
    NesClockFilter,
    NesGameGenieCode,
    SnesLatchFilter,
    SnesClockFilter,
    SnesGameGenieCode,
    SnesLatchTrain,
    GenesisGameGenieCode,
    InputChunk,
    InputMoment,
    Transition,
    LagFrameChunk,
    MovieTransition,
    Comment,
    Experimental,
    Unspecified,
    Unsupported,
}



////////////////////////////////////// Unsupported //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Unsupported {
    pub key: Vec<u8>,
    pub payload: Vec<u8>,
}
impl Decode for Unsupported {
    fn decode(key: &[u8], payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            key: key.to_vec(),
            payload: payload.to_vec(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Unsupported
    }
}
impl Encode for Unsupported {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_slice(&self.payload);
        
        w.into_packet(&self.key, keylen)
    }
    
    fn key(&self) -> Vec<u8> {
        self.key.clone()
    }
}


////////////////////////////////////// CONSOLE_TYPE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleType {
    pub kind: u8,
    pub custom: Option<String>,
}
impl Decode for ConsoleType {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 1 {
            return Err(PacketError::invalid(key, payload));
        }
        let kind = payload.read_u8();
        
        let custom = payload.read_remaining();
        let custom = if custom.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(custom).to_string())
        };
        
        Ok(Self {
            kind,
            custom,
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::ConsoleType
    }
}
impl Encode for ConsoleType {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.kind);
        w.write_option_string(&self.custom);
        
        w.into_packet(&self.key(), keylen)
    }
    
    fn key(&self) -> Vec<u8> {
        KEY_CONSOLE_TYPE.to_vec()
    }
}


////////////////////////////////////// CONSOLE_REGION //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleRegion {
    pub region: u8,
}
impl Decode for ConsoleRegion {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            region: payload.read_u8(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::ConsoleRegion
    }
}
impl Encode for ConsoleRegion {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.region);
        
        w.into_packet(&self.key(), keylen)
    }
    
    fn key(&self) -> Vec<u8> {
        KEY_CONSOLE_REGION.to_vec()
    }
}


////////////////////////////////////// GAME_TITLE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct GameTitle {
    pub title: String,
}
impl Decode for GameTitle {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            title: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::GameTitle
    }
}
impl Encode for GameTitle {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.title);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_GAME_TITLE.to_vec()
    }
}


////////////////////////////////////// ROM_NAME //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct RomName {
    pub name: String,
}
impl Decode for RomName {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            name: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::RomName
    }
}
impl Encode for RomName {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.name);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_ROM_NAME.to_vec()
    }
}


////////////////////////////////////// ATTRIBUTION //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Attribution {
    pub kind: u8,
    pub name: String,
}
impl Decode for Attribution {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            kind: payload.read_u8(),
            name: payload.read_string(payload.remaining()),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Attribution
    }
}
impl Encode for Attribution {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.kind);
        w.write_str(&self.name);
        
        w.into_packet(&self.key(), keylen)
    }
    
    fn key(&self) -> Vec<u8> {
        KEY_ATTRIBUTION.to_vec()
    }
}


////////////////////////////////////// CATEGORY //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub category: String,
}
impl Decode for Category {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            category: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Category
    }
}
impl Encode for Category {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.category);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_CATEGORY.to_vec()
    }
}


////////////////////////////////////// EMULATOR_NAME //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct EmulatorName {
    pub name: String,
}
impl Decode for EmulatorName {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            name: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::EmulatorName
    }
}
impl Encode for EmulatorName {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.name);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_EMULATOR_NAME.to_vec()
    }
}


////////////////////////////////////// EMULATOR_VERSION //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct EmulatorVersion {
    pub version: String,
}
impl Decode for EmulatorVersion {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            version: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::EmulatorVersion
    }
}
impl Encode for EmulatorVersion {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.version);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_EMULATOR_VERSION.to_vec()
    }
}


////////////////////////////////////// EMULATOR_CORE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct EmulatorCore {
    pub core: String,
}
impl Decode for EmulatorCore {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            core: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::EmulatorCore
    }
}
impl Encode for EmulatorCore {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.core);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_EMULATOR_CORE.to_vec()
    }
}


////////////////////////////////////// TAS_LAST_MODIFIED //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct TasLastModified {
    pub epoch: i64,
}
impl Decode for TasLastModified {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 8 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            epoch: payload.read_i64(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::TasLastModified
    }
}
impl Encode for TasLastModified {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_i64(self.epoch);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_TAS_LAST_MODIFIED.to_vec()
    }
}


////////////////////////////////////// DUMP_CREATED //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct DumpCreated {
    pub epoch: i64,
}
impl Decode for DumpCreated {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 8 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            epoch: payload.read_i64(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::DumpCreated
    }
}
impl Encode for DumpCreated {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_i64(self.epoch);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_DUMP_CREATED.to_vec()
    }
}


////////////////////////////////////// DUMP_LAST_MODIFIED //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct DumpLastModified {
    pub epoch: i64,
}
impl Decode for DumpLastModified {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 8 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            epoch: payload.read_i64(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::DumpLastModified
    }
}
impl Encode for DumpLastModified {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_i64(self.epoch);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_DUMP_LAST_MODIFIED.to_vec()
    }
}


////////////////////////////////////// TOTAL_FRAMES //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct TotalFrames {
    pub frames: u32,
}
impl Decode for TotalFrames {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 4 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            frames: payload.read_u32(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::TotalFrames
    }
}
impl Encode for TotalFrames {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u32(self.frames);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_TOTAL_FRAMES.to_vec()
    }
}


////////////////////////////////////// RERECORDS //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Rerecords {
    pub rerecords: u32,
}
impl Decode for Rerecords {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 4 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            rerecords: payload.read_u32(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Rerecords
    }
}
impl Encode for Rerecords {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u32(self.rerecords);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_RERECORDS.to_vec()
    }
}


////////////////////////////////////// SOURCE_LINK //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLink {
    pub link: String,
}
impl Decode for SourceLink {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            link: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::SourceLink
    }
}
impl Encode for SourceLink {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.link);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_SOURCE_LINK.to_vec()
    }
}


////////////////////////////////////// BLANK_FRAMES //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct BlankFrames {
    pub frames: i16,
}
impl Decode for BlankFrames {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 2 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            frames: payload.read_i16(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::BlankFrames
    }
}
impl Encode for BlankFrames {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_i16(self.frames);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_BLANK_FRAMES.to_vec()
    }
}


////////////////////////////////////// VERIFIED //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Verified {
    pub verified: bool,
}
impl Decode for Verified {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            verified: payload.read_bool(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Verified
    }
}
impl Encode for Verified {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_bool(self.verified);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_VERIFIED.to_vec()
    }
}


////////////////////////////////////// MEMORY_INIT //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryInit {
    pub data_type: u8,
    pub device: u16,
    pub required: bool,
    pub name: String,
    pub data: Option<Vec<u8>>,
}
impl Decode for MemoryInit {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 5 {
            return Err(PacketError::invalid(key, payload));
        }
        let data_type = payload.read_u8();
        let device = payload.read_u16();
        let required = payload.read_bool();
        
        let nlen = payload.read_u8();
        if payload.remaining() < nlen as usize {
            return Err(PacketError::invalid(key, payload));
        }
        let name = payload.read_string(nlen as usize);
        
        Ok(Self {
            data_type,
            device,
            required,
            name,
            data: if data_type == 0xFF { Some(payload.read_remaining().to_vec()) } else { None },
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::MemoryInit
    }
}
impl Encode for MemoryInit {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.data_type);
        w.write_u16(self.device);
        w.write_bool(self.required);
        w.write_u8_str(&self.name);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_MEMORY_INIT.to_vec()
    }
}


////////////////////////////////////// GAME_IDENTIFIER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct GameIdentifier {
    pub kind: u8,
    pub encoding: u8,
    pub name: String,
    pub identifier: Vec<u8>,
}
impl Decode for GameIdentifier {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 3 {
            return Err(PacketError::invalid(key, payload));
        }
        
        let kind = payload.read_u8();
        let encoding = payload.read_u8();
        
        let nlen = payload.read_u8();
        if payload.remaining() < nlen as usize {
            return Err(PacketError::invalid(key, payload));
        }
        let name = payload.read_string(nlen as usize);
        
        let identifier = payload.read_remaining().to_vec();
        
        Ok(Self {
            kind,
            encoding,
            name,
            identifier,
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::GameIdentifier
    }
}
impl Encode for GameIdentifier {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.kind);
        w.write_u8(self.encoding);
        w.write_u8_str(&self.name);
        w.write_slice(&self.identifier);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_GAME_IDENTIFIER.to_vec()
    }
}


////////////////////////////////////// MOVIE_LICENSE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct MovieLicense {
    pub license: String,
}
impl Decode for MovieLicense {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            license: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::MovieLicense
    }
}
impl Encode for MovieLicense {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.license);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_MOVIE_LICENSE.to_vec()
    }
}


////////////////////////////////////// MOVIE_FILE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct MovieFile {
    pub name: String,
    pub data: Vec<u8>,
}
impl Decode for MovieFile {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 1 {
            return Err(PacketError::invalid(key, payload));
        }
        let nlen = payload.read_u8();
        if payload.remaining() < nlen as usize {
            return Err(PacketError::invalid(key, payload));
        }
        let name = payload.read_string(nlen as usize);
        
        Ok(Self {
            name,
            data: payload.read_remaining().to_vec(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::MovieFile
    }
}
impl Encode for MovieFile {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8_str(&self.name);
        w.write_slice(&self.data);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_MOVIE_FILE.to_vec()
    }
}


////////////////////////////////////// PORT_CONTROLLER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct PortController {
    pub port: u8,
    pub kind: u16,
}
impl Decode for PortController {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 3 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            port: payload.read_u8(),
            kind: payload.read_u16(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::PortController
    }
}
impl Encode for PortController {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.port);
        w.write_u16(self.kind);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_PORT_CONTROLLER.to_vec()
    }
}


////////////////////////////////////// PORT_OVERREAD //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct PortOverread {
    pub port: u8,
    pub overread: bool,
}
impl Decode for PortOverread {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 2 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            port: payload.read_u8(),
            overread: payload.read_bool(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::PortOverread
    }
}
impl Encode for PortOverread {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.port);
        w.write_bool(self.overread);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_PORT_OVERREAD.to_vec()
    }
}


////////////////////////////////////// NES_LATCH_FILTER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct NesLatchFilter {
    pub time: u16,
}
impl Decode for NesLatchFilter {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 2 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            time: payload.read_u16(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::NesLatchFilter
    }
}
impl Encode for NesLatchFilter {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u16(self.time);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_NES_LATCH_FILTER.to_vec()
    }
}


////////////////////////////////////// NES_CLOCK_FILTER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct NesClockFilter {
    pub time: u8,
}
impl Decode for NesClockFilter {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            time: payload.read_u8(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::NesClockFilter
    }
}
impl Encode for NesClockFilter {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.time);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_NES_CLOCK_FILTER.to_vec()
    }
}


////////////////////////////////////// NES_GAME_GENIE_CODE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct NesGameGenieCode {
    pub code: String,
}
impl Decode for NesGameGenieCode {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            code: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::NesGameGenieCode
    }
}
impl Encode for NesGameGenieCode {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.code);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_NES_GAME_GENIE_CODE.to_vec()
    }
}


////////////////////////////////////// SNES_LATCH_FILTER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct SnesLatchFilter {
    pub time: u16,
}
impl Decode for SnesLatchFilter {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 2 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            time: payload.read_u16(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::SnesLatchFilter
    }
}
impl Encode for SnesLatchFilter {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u16(self.time);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_SNES_LATCH_FILTER.to_vec()
    }
}


////////////////////////////////////// SNES_CLOCK_FILTER //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct SnesClockFilter {
    pub time: u8,
}
impl Decode for SnesClockFilter {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            time: payload.read_u8(),
        })
    }

    fn kind(&self) -> PacketKind {
        PacketKind::SnesClockFilter
    }
}
impl Encode for SnesClockFilter {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.time);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_SNES_CLOCK_FILTER.to_vec()
    }
}


////////////////////////////////////// SNES_GAME_GENIE_CODE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct SnesGameGenieCode {
    pub code: String,
}
impl Decode for SnesGameGenieCode {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            code: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::SnesGameGenieCode
    }
}
impl Encode for SnesGameGenieCode {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.code);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_SNES_GAME_GENIE_CODE.to_vec()
    }
}


////////////////////////////////////// SNES_LATCH_TRAIN //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct SnesLatchTrain {
    pub points: Vec<u64>,
}
impl Decode for SnesLatchTrain {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            points: payload.read_remaining()
                .chunks_exact(8)
                .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
                .collect()
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::SnesLatchTrain
    }
}
impl Encode for SnesLatchTrain {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_slice(&self.points.iter()
            .map(|point| point.to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>());
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_SNES_LATCH_TRAIN.to_vec()
    }
}


////////////////////////////////////// GENESIS_GAME_GENIE_CODE //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct GenesisGameGenieCode {
    pub code: String,
}
impl Decode for GenesisGameGenieCode {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            code: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::GenesisGameGenieCode
    }
}
impl Encode for GenesisGameGenieCode {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.code);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_GENESIS_GAME_GENIE_CODE.to_vec()
    }
}


////////////////////////////////////// INPUT_CHUNK //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct InputChunk {
    pub port: u8,
    pub inputs: Vec<u8>,
}
impl Decode for InputChunk {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            port: payload.read_u8(),
            inputs: payload.read_remaining().to_vec(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::InputChunk
    }
}
impl Encode for InputChunk {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.port);
        w.write_slice(&self.inputs);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_INPUT_CHUNK.to_vec()
    }
}


////////////////////////////////////// INPUT_MOMENT //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct InputMoment {
    pub port: u8,
    pub hold: bool,
    pub index_type: u8,
    pub index: u64,
    pub inputs: Vec<u8>,
}
impl Decode for InputMoment {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 11 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            port: payload.read_u8(),
            hold: payload.read_bool(),
            index_type: payload.read_u8(),
            index: payload.read_u64(),
            inputs: payload.read_remaining().to_vec(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::InputMoment
    }
}
impl Encode for InputMoment {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.port);
        w.write_bool(self.hold);
        w.write_u8(self.index_type);
        w.write_u64(self.index);
        w.write_slice(&self.inputs);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_INPUT_MOMENT.to_vec()
    }
}


////////////////////////////////////// TRANSITION //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub port: u8,
    pub index_type: u8,
    pub index: u64,
    pub transition_type: u8,
    pub packet: Option<Box<Packet>>,
}
impl Decode for Transition {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 11 {
            return Err(PacketError::invalid(key, payload));
        }
        let port = payload.read_u8();
        let index_type = payload.read_u8();
        let index = payload.read_u64();
        let transition_type = payload.read_u8();
        let packet_data = payload.read_remaining();
        let mut packet_reader = Reader::new(&packet_data);
        
        Ok(Self {
            port,
            index_type,
            index,
            transition_type,
            packet: if transition_type == 0xFF { Some(Box::new(Packet::with_reader(&mut packet_reader, key.len() as u8)?)) } else { None }
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Transition
    }
}
impl Encode for Transition {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u8(self.port);
        w.write_u8(self.index_type);
        w.write_u64(self.index);
        w.write_u8(self.transition_type);
        if let Some(packet) = self.packet.as_ref() {
            w.write_slice(&packet.encode(keylen));
        }
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_TRANSITION.to_vec()
    }
}


////////////////////////////////////// LAG_FRAME_CHUNK //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct LagFrameChunk {
    pub movie_frame: u32,
    pub count: u32,
}
impl Decode for LagFrameChunk {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 8 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            movie_frame: payload.read_u32(),
            count: payload.read_u32(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::LagFrameChunk
    }
}
impl Encode for LagFrameChunk {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u32(self.movie_frame);
        w.write_u32(self.count);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_LAG_FRAME_CHUNK.to_vec()
    }
}


////////////////////////////////////// MOVIE_TRANSITION //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct MovieTransition {
    pub movie_frame: u32,
    pub transition_type: u8,
    pub packet: Option<Box<Packet>>,
}
impl Decode for MovieTransition {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() < 5 {
            return Err(PacketError::invalid(key, payload));
        }
        let movie_frame = payload.read_u32();
        let transition_type = payload.read_u8();
        let packet_data = payload.read_remaining();
        let mut packet_reader = Reader::new(&packet_data);
        
        Ok(Self {
            movie_frame,
            transition_type,
            packet: if transition_type == 0xFF { Some(Box::new(Packet::with_reader(&mut packet_reader, key.len() as u8)?)) } else { None }
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::MovieTransition
    }
}
impl Encode for MovieTransition {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_u32(self.movie_frame);
        w.write_u8(self.transition_type);
        if let Some(packet) = self.packet.as_ref() {
            w.write_slice(&packet.encode(keylen));
        }
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_MOVIE_TRANSITION.to_vec()
    }
}


////////////////////////////////////// COMMENT //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub comment: String,
}
impl Decode for Comment {
    fn decode(_key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            comment: payload.read_string(payload.remaining())
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Comment
    }
}
impl Encode for Comment {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_str(&self.comment);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_COMMENT.to_vec()
    }
}


////////////////////////////////////// EXPERIMENTAL //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Experimental {
    pub experimental: bool,
}
impl Decode for Experimental {
    fn decode(key: &[u8], mut payload: Reader) -> Result<Self, PacketError> {
        if payload.remaining() != 1 {
            return Err(PacketError::invalid(key, payload));
        }
        
        Ok(Self {
            experimental: payload.read_bool(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Experimental
    }
}
impl Encode for Experimental {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_bool(self.experimental);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_EXPERIMENTAL.to_vec()
    }
}


////////////////////////////////////// UNSPECIFIED //////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct Unspecified {
    pub payload: Vec<u8>,
}
impl Decode for Unspecified {
    fn decode(_key: &[u8], payload: Reader) -> Result<Self, PacketError> {
        Ok(Self {
            payload: payload.to_vec(),
        })
    }
    
    fn kind(&self) -> PacketKind {
        PacketKind::Unspecified
    }
}
impl Encode for Unspecified {
    fn encode(&self, keylen: u8) -> Vec<u8> {
        let mut w = Writer::new();
        
        w.write_slice(&self.payload);
        
        w.into_packet(&self.key(), keylen)
    }

    fn key(&self) -> Vec<u8> {
        KEY_UNSPECIFIED.to_vec()
    }
}
