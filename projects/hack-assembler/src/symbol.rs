use std::collections::HashMap;

use phf::phf_map;

pub(super) static PREDEFINED: phf::Map<&'static str, u16> = phf_map! {
    "SP" => 0x0,
    "LCL" => 0x1,
    "ARG" => 0x2,
    "THIS" => 0x3,
    "THAT" => 0x4,
    "R0" => 0x0,
    "R1" => 0x1,
    "R2" => 0x2,
    "R3" => 0x3,
    "R4" => 0x4,
    "R5" => 0x5,
    "R6" => 0x6,
    "R7" => 0x7,
    "R8" => 0x8,
    "R9" => 0x9,
    "R10" => 0xa,
    "R11" => 0xb,
    "R12" => 0xc,
    "R13" => 0xd,
    "R14" => 0xe,
    "R15" => 0xf,
    "SCREEN" => 0x4000,
    "KBD" => 0x6000,
};

pub(super) struct Labels<'a>(pub HashMap<&'a str, u16>);

pub(super) const VARIABLES_RAM_OFFSET: u16 = 16;

pub(super) struct Variables<'a>(pub HashMap<&'a str, u16>);
