#[cfg(test)]
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;

pub use code::translate;
pub use parser::{parse, Instruction};

mod code;
mod parser;
mod symbol;
