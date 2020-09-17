use phf::phf_map;

pub(super) static COMP: phf::Map<&'static str, u16> = phf_map! {
    "0" => 0b0101010u16,
    "1" => 0b0111111u16,
    "-1" => 0b111010u16,
    "D" => 0b0001100u16,
    "A" => 0b0110000u16,
    "M" => 0b1110000u16,
    "!D" => 0b0001101u16,
    "!A" => 0b0110001u16,
    "!M" => 0b1110001u16,
    "-D" => 0b0001111u16,
    "-A" => 0b0110011u16,
    "-M" => 0b1110011u16,
    "D+1" => 0b0011111u16,
    "A+1" => 0b0110111u16,
    "M+1" => 0b1110111u16,
    "D-1" => 0b0001110u16,
    "A-1" => 0b0110010u16,
    "M-1" => 0b1110010u16,
    "D+A" => 0b0000010u16,
    "D+M" => 0b1000010u16,
    "D-A" => 0b0010011u16,
    "D-M" => 0b1010011u16,
    "A-D" => 0b0000111u16,
    "M-D" => 0b1000111u16,
    "D&A" => 0b0000000u16,
    "D&M" => 0b1000000u16,
    "D|A" => 0b0010101u16,
    "D|M" => 0b1010101u16,
};

pub(super) static DEST: phf::Map<&'static str, u16> = phf_map! {
    "null" => 0b000u16,
    "M" => 0b001u16,
    "D" => 0b010u16,
    "MD" => 0b011u16,
    "A" => 0b100u16,
    "AM" => 0b101u16,
    "AD" => 0b110u16,
    "AMD" => 0b111u16,
};

pub(super) static JUMP: phf::Map<&'static str, u16> = phf_map! {
    "null" => 0b000u16,
    "JGT" => 0b001u16,
    "JEQ" => 0b010u16,
    "JGE" => 0b011u16,
    "JLT" => 0b100u16,
    "JNE" => 0b101u16,
    "JLE" => 0b110u16,
    "JMP" => 0b111u16,
};
