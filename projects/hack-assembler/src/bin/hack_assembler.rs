use std::{
    borrow::Cow,
    fs,
    io::{Read, Result, Write},
    path::{Path, PathBuf},
    process::exit,
};

use structopt::StructOpt;

use hack_assembler::{parse, translate};

#[derive(Debug, StructOpt)]
#[structopt(name = "hack-assembler")]
struct Opt {
    #[structopt(
        long,
        help = "Pathname to an input file containing a Hack assembly program",
        parse(from_os_str)
    )]
    input: PathBuf,
    #[structopt(
        long,
        help = "Pathname to an output file containing the translated Hack machine code (defaults to the same location as input)",
        parse(from_os_str)
    )]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let mut file = fs::File::open(&opt.input)?;
    let mut asm = String::new();
    file.read_to_string(&mut asm)?;

    let instructions = parse(&asm);
    let translated = translate(instructions);
    write_binary(translated, output(&opt))?;

    Ok(())
}

fn write_binary<P: AsRef<Path>>(translated: Vec<u16>, output: P) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;

    for machine_code in translated {
        writeln!(file, "{:016b}", machine_code)?;
    }

    Ok(())
}

fn output(opt: &Opt) -> Cow<'_, Path> {
    match &opt.output {
        Some(result) => Cow::Borrowed(result),
        None => {
            let mut result = opt.input.to_path_buf();
            result.set_extension("hack");
            result.into()
        }
    }
}
