
pub fn console_type_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "NES",
        0x02 => "SNES",
        0x03 => "N64",
        0x04 => "GC",
        0x05 => "GB",
        0x06 => "GBC",
        0x07 => "GBA",
        0x08 => "Genesis",
        0x09 => "A2600",
        0xFF => "Custom",
        _ => return None
    }.into())
}

pub fn console_region_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "NTSC",
        0x02 => "PAL",
        _ => return None
    }.into())
}

pub fn attribution_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "Author",
        0x02 => "Verifier",
        0x03 => "TASD File Creator",
        0x04 => "TASD File Editor",
        0xFF => "Other",
        _ => return None
    }.into())
}

pub fn memory_init_data_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "No initialization required",
        0x02 => "All 0x00",
        0x03 => "All 0xFF",
        0x04 => "00 00 00 00 FF FF FF FF (repeating)",
        0x05 => "Random",
        0xFF => "Custom",
        _ => return None
    }.into())
}

pub fn memory_init_device_lut(kind: u16) -> Option<String> {
    Some(match kind {
        0x0101 => "NES CPU RAM",
        0x0102 => "NES Cartridge Save Data",
        0x0201 => "SNES CPU RAM",
        0x0202 => "SNES Cartridge Save Data",
        0x0501 => "GB CPU RAM",
        0x0502 => "GB Cartridge Save Data",
        0x0601 => "GBC CPU RAM",
        0x0602 => "GBC Cartridge Save Data",
        0x0701 => "GBA CPU RAM",
        0x0702 => "GBA Cartridge Save Data",
        0x0801 => "Genesis CPU RAM",
        0x0802 => "Genesis Cartridge Save Data",
        0x0901 => "A2600 CPU RAM",
        0x0902 => "A2600 Cartridge Save Data",
        0xFFFF => "Custom/Other Device",
        _ => return None
    }.into())
}

pub fn game_identifier_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "MD5 Hash",
        0x02 => "SHA1 Hash",
        0x03 => "SHA224 Hash",
        0x04 => "SHA256 Hash",
        0x05 => "SHA384 Hash",
        0x06 => "SHA512 Hash",
        0x07 => "SHA512/224 Hash",
        0x08 => "SHA512/256 Hash",
        0x09 => "SHA3-224 Hash",
        0x0A => "SHA3-256 Hash",
        0x0B => "SHA3-384 Hash",
        0x0C => "SHA3-512 Hash",
        0x0D => "SHAKE-128 Hash",
        0x0E => "SHAKE-256 Hash",
        0xFF => "Other",
        _ => return None
    }.into())
}

pub fn identifier_encoding_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "Raw Binary",
        0x02 => "Base 16 (Case Insensitive)",
        0x03 => "Base 32 (Case Insensitive)",
        0x04 => "Base 64",
        _ => return None
    }.into())
}

pub fn controller_type_lut(kind: u16) -> Option<String> {
    Some(match kind {
        0x0101 => "NES Standard Controller",
        0x0102 => "NES Four Score",
        0x0103 => "(RESERVED) NES Zapper",
        0x0104 => "(RESERVED) NES Power Pad",
        0x0105 => "(RESERVED) Famicom Family BASIC Keyboard",
        0x0201 => "SNES Standard Controller",
        0x0202 => "SNES Super Multitap",
        0x0203 => "SNES Mouse",
        0x0204 => "(RESERVED) SNES Superscope",
        0x0301 => "N64 Standard Controller",
        0x0302 => "N64 Standard Controller with Rumble Pak",
        0x0303 => "N64 Standard Controller with Controller Pak",
        0x0304 => "N64 Standard Controller with Transfer Pak",
        0x0305 => "N64 Mouse",
        0x0306 => "(RESERVED) N64 Voice Recognition Unit (VRU)",
        0x0307 => "(RESERVED) N64 RandNet Keyboard",
        0x0308 => "N64 Densha de Go",
        0x0401 => "GC Standard Controller",
        0x0402 => "(RESERVED) GC Keyboard",
        0x0501 => "GB Gamepad",
        0x0601 => "GBC Gamepad",
        0x0701 => "GBA Gamepad",
        0x0801 => "Genesis (Mega Drive) 3-Button",
        0x0802 => "Genesis (Mega Drive) 6-Button",
        0x0901 => "A2600 Joystick",
        0x0902 => "(RESERVED) A2600 Paddle",
        0x0903 => "A2600 Keyboard Controller",
        0xFFFF => "Other/Unspecified",
        _ => return None
    }.into())
}

pub fn input_moment_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "Frame",
        0x02 => "Cycle Count",
        0x03 => "Milliseconds",
        0x04 => "Microseconds * 10",
        _ => return None
    }.into())
}

pub fn transition_index_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "Frame",
        0x02 => "Cycle Count",
        0x03 => "Milliseconds",
        0x04 => "Microseconds",
        0x05 => "Nanoseconds",
        0x06 => "INPUT_CHUNK Index",
        _ => return None
    }.into())
}

pub fn transition_kind_lut(kind: u8) -> Option<String> {
    Some(match kind {
        0x01 => "Soft Reset",
        0x02 => "Power Reset",
        0x03 => "Restart TASD File",
        0xFF => "Packet Derived",
        _ => return None
    }.into())
}