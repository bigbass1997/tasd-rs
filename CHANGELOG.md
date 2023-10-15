# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]
- Changed path types to use `camino` UTF-8 enforced types

## [0.4.0] - 2023-10-13
- Added strum EnumString to PacketKind to allow converting to/from a string
- Added method for writing a string prefixed with a u8 length byte
- Updated GAME_IDENTIFIER to include a name
- Changed lookup functions to return an owned value

## [0.3.0] - 2023-09-28
- Added unit and integration tests (in-progress)
- Fixed bug where payload lengths larger than 255 bytes were decoded incorrectly
- Removed NES_OVERREAD and SNES_OVERREAD
- Added PORT_OVERREAD
- Changed TRANSITION to include a port number

## [0.2.1] - 2023-07-12
- Changed `TasdFile::new()` to include the DUMP_CREATED packet in the returned `TasdFile` instance
- Changed PacketKind's Display implementation to return the name in SNAKE_CASE
- Added `From<T> for Packet` implementations for all packet structs

## [0.2.0] - 2023-07-06
- Changed packet data structures
- Added SNES_LATCH_TRAIN packet
