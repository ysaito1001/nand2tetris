use std::collections::HashMap;

use crate::parser::Instruction;
use crate::symbol::{Labels, Variables, PREDEFINED, VARIABLES_RAM_OFFSET};

use c_translation::{COMP, DEST, JUMP};
mod c_translation;

pub fn translate(instructions: Vec<Instruction>) -> Vec<u16> {
    let mut labels = Labels { 0: HashMap::new() };
    add_entry_for(&mut labels, &instructions);

    let mut variables = Variables { 0: HashMap::new() };
    let mut result: Vec<u16> = Vec::new();

    for instruction in instructions {
        let machine_code = match instruction {
            Instruction::A(symbol) => translate_a_instruction(symbol, &labels, &mut variables),
            Instruction::C { dest, comp, jump } => translate_c_instruction(dest, comp, jump),
            _ => continue,
        };
        result.push(machine_code);
    }

    result
}

fn add_entry_for<'a>(labels: &mut Labels<'a>, instructions: &[Instruction<'a>]) {
    let mut pc: u16 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::L(label) => {
                labels.0.insert(label, pc);
            }
            _ => pc += 1,
        }
    }
}

fn translate_a_instruction<'a>(
    symbol: &'a str,
    labels: &Labels<'a>,
    variables: &mut Variables<'a>,
) -> u16 {
    if let Result::Ok(number) = symbol.parse::<u16>() {
        return number;
    }

    if PREDEFINED.contains_key(symbol) {
        return PREDEFINED[symbol];
    }

    if labels.0.contains_key(symbol) {
        return labels.0[symbol];
    }

    if !variables.0.contains_key(symbol) {
        variables
            .0
            .insert(symbol, variables.0.len() as u16 + VARIABLES_RAM_OFFSET);
    }

    variables.0[symbol]
}

fn translate_c_instruction(dest: Option<&str>, comp: &str, jump: Option<&str>) -> u16 {
    0b111u16 * 2_u16.pow(13)
        + COMP[comp] * 2_u16.pow(6)
        + dest.map_or(0_u16, |d| DEST[d]) * 2_u16.pow(3)
        + jump.map_or(0_u16, |j| JUMP[j])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn adds_entry_for_labels() {
        let label = "LOOP";
        let instructions = vec![
            Instruction::A("@i"),
            Instruction::C {
                dest: Some("M"),
                comp: "1",
                jump: None,
            },
            Instruction::A("@sum"),
            Instruction::C {
                dest: Some("M"),
                comp: "0",
                jump: None,
            },
            Instruction::L(label),
            Instruction::A("@i"),
        ];
        let mut labels = Labels { 0: HashMap::new() };
        add_entry_for(&mut labels, &instructions);
        assert_eq!(labels.0.get_key_value(label), Some((&label, &4_u16)));
    }

    #[test]
    fn translates_a_instruction_with_number() {
        assert_eq!(
            translate_a_instruction(
                "1",
                &Labels { 0: HashMap::new() },
                &mut Variables { 0: HashMap::new() },
            ),
            0x0001
        );
    }

    #[test]
    fn translates_a_instruction_with_predefined() {
        assert_eq!(
            translate_a_instruction(
                "SCREEN",
                &Labels { 0: HashMap::new() },
                &mut Variables { 0: HashMap::new() },
            ),
            0x4000
        );
    }

    #[test]
    fn translates_a_instruction_with_label() {
        assert_eq!(
            translate_a_instruction(
                "LOOP",
                &Labels {
                    0: hashmap! { "LOOP" => 4 as u16,}
                },
                &mut Variables { 0: HashMap::new() },
            ),
            0x0004
        );
    }

    #[test]
    fn translates_a_instruction_with_new_variable() {
        let i = "i";
        let mut variables = Variables { 0: HashMap::new() };
        assert_eq!(
            translate_a_instruction(i, &Labels { 0: HashMap::new() }, &mut variables,),
            0x0010
        );
        assert_eq!(variables.0.get_key_value(i), Some((&i, &0x0010)));
    }

    #[test]
    fn translates_a_instruction_with_existing_variable() {
        let i = "i";
        assert_eq!(
            translate_a_instruction(
                i,
                &Labels { 0: HashMap::new() },
                &mut Variables {
                    0: hashmap! { i => 16 as u16}
                },
            ),
            0x0010
        );
    }

    #[test]
    fn translates_c_instruction() {
        assert_eq!(
            translate_c_instruction(None, "D-M", None),
            0b1111_0100_1100_0000
        );
    }

    #[test]
    fn translates_c_instruction_with_dest() {
        assert_eq!(
            translate_c_instruction(Some("A"), "D-M", None),
            0b1111_0100_1110_0000
        );
    }

    #[test]
    fn translates_c_instruction_with_jump() {
        assert_eq!(
            translate_c_instruction(None, "D-M", Some("JGT")),
            0b1111_0100_1100_0001
        );
    }

    #[test]
    fn translates_c_instruction_with_all() {
        assert_eq!(
            translate_c_instruction(Some("A"), "D-M", Some("JGT")),
            0b1111_0100_1110_0001
        );
    }
}
