use nom::{combinator::rest, IResult};

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    A(&'a str),
    C {
        dest: Option<&'a str>,
        comp: &'a str,
        jump: Option<&'a str>,
    },
    L(&'a str),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(needs_parsing)
        .map(remove_trailing_comments)
        .map(|line| match instruction(line) {
            IResult::Ok((_, output)) => output,
            IResult::Err(_) => unreachable!(),
        })
        .collect::<Vec<Instruction>>()
}

fn needs_parsing(line: &&str) -> bool {
    let line = line.trim();
    !line.is_empty() && !line.starts_with("//")
}

fn remove_trailing_comments(line: &str) -> &str {
    line.split("//").next().unwrap().trim()
}

named!(instruction<&str, Instruction>,
    alt!(
        a_instruction |
        c_instruction |
        l_instruction
    )
);

named!(a_instruction<&str, Instruction>,
    do_parse!(
        tag!("@") >>
        symbol: rest >>
        (Instruction::A(symbol))
    )
);

named!(c_instruction<&str, Instruction>,
    do_parse!(
        dest: opt!(complete!(terminated!(is_a!("ADM"), tag!("=")))) >>
        comp: complete!(comp_field) >>
        jump: opt!(complete!(preceded!(tag!(";"), rest))) >>
        (Instruction::C{dest, comp, jump})
    )
);

named!(comp_field<&str, &str>,
    do_parse!(
        comp: alt!(
            complete!(tag!("0")) |
            complete!(tag!("1")) |
            complete!(tag!("-1")) |
            complete!(tag!("D+1")) |
            complete!(tag!("A+1")) |
            complete!(tag!("M+1")) |
            complete!(tag!("D-1")) |
            complete!(tag!("A-1")) |
            complete!(tag!("M-1")) |
            complete!(tag!("D+A")) |
            complete!(tag!("D+M")) |
            complete!(tag!("D-A")) |
            complete!(tag!("D-M")) |
            complete!(tag!("A-D")) |
            complete!(tag!("M-D")) |
            complete!(tag!("D&A")) |
            complete!(tag!("D&M")) |
            complete!(tag!("D|A")) |
            complete!(tag!("D|M")) |
            complete!(tag!("D")) |
            complete!(tag!("A")) |
            complete!(tag!("M")) |
            complete!(tag!("!D")) |
            complete!(tag!("!A")) |
            complete!(tag!("!M")) |
            complete!(tag!("-D")) |
            complete!(tag!("-A")) |
            complete!(tag!("-M"))
        ) >>
        (comp)
    )
);

named!(l_instruction<&str, Instruction>,
    do_parse!(
        label: delimited!(tag!("("), take_until!(")"), tag!(")")) >>
        (Instruction::L(label))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_instruction_with_symbol() {
        assert_eq!(parse("@R0"), vec![Instruction::A("R0")]);
    }

    #[test]
    fn parses_a_instruction_with_decimal() {
        assert_eq!(parse("@2"), vec![Instruction::A("2")]);
    }

    #[test]
    fn parses_a_instruction_with_comments() {
        assert_eq!(parse("@R0   // A = RAM[0]"), vec![Instruction::A("R0")]);
    }

    #[test]
    fn parses_c_instruction() {
        assert_eq!(
            parse("D-M"),
            vec![Instruction::C {
                dest: None,
                comp: "D-M",
                jump: None,
            }]
        )
    }

    #[test]
    fn parses_c_instruction_with_dest() {
        assert_eq!(
            parse("A=D-M"),
            vec![Instruction::C {
                dest: Some("A"),
                comp: "D-M",
                jump: None,
            }]
        )
    }

    #[test]
    fn parses_c_instruction_with_jump() {
        assert_eq!(
            parse("D-M;JGT"),
            vec![Instruction::C {
                dest: None,
                comp: "D-M",
                jump: Some("JGT"),
            }]
        )
    }

    #[test]
    fn parses_c_instruction_with_all() {
        assert_eq!(
            parse("A=D-M;JGT"),
            vec![Instruction::C {
                dest: Some("A"),
                comp: "D-M",
                jump: Some("JGT"),
            }]
        )
    }

    #[test]
    fn parses_c_instruction_with_comments() {
        assert_eq!(
            parse("A=D-M;JGT   // if D - M > 0 goto A"),
            vec![Instruction::C {
                dest: Some("A"),
                comp: "D-M",
                jump: Some("JGT"),
            }]
        )
    }

    #[test]
    fn parses_l_instruction() {
        assert_eq!(parse("(LOOP)"), vec![Instruction::L("LOOP")])
    }

    #[test]
    fn parses_l_instruction_with_comments() {
        assert_eq!(
            parse("(LOOP)   // declare the label LOOP"),
            vec![Instruction::L("LOOP")]
        )
    }
}
