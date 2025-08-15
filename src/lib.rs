use clap::Parser;
use pixel_bender::{
    assembly::PixelBenderShaderAssembly, disassembly::PixelBenderShaderDisassembly, parse_shader,
};
use std::io::{IsTerminal, Write};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, PbasmError>;

#[derive(Error, Debug)]
pub enum PbasmError {
    #[error("Disassembly error: {0}")]
    DisassemblyError(#[from] pixel_bender::PixelBenderParsingError),

    #[error("Assembly error: {0}")]
    AssemblyError(#[from] pixel_bender::assembly::PbasmError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Cowardly refusing to output binary data to a terminal")]
    BinaryDataInTerminalError,
}

#[derive(Parser, Debug)]
#[clap(name = "pbasm", author, version)]
pub struct Opt {
    /// Path to the source file to either assemble or disassemble.
    #[clap(name = "source")]
    pub source: String,

    /// Path to the output file. If not specified, stdout is assumed.
    #[clap(short = 'o', long = "output")]
    pub output: Option<String>,

    /// When set, pbasm will perform disassembly instead of assembly.
    #[clap(short = 'd', long = "disassemble")]
    pub disassemble: bool,
}

fn disassemble(opt: Opt, write: &mut dyn Write) -> Result<()> {
    let data = std::fs::read(&opt.source)?;
    let parsed = parse_shader(&data, false)?;
    write!(write, "{}", PixelBenderShaderDisassembly(&parsed))?;
    Ok(())
}

fn assemble(opt: Opt, write: &mut dyn Write) -> Result<()> {
    let input = std::fs::read_to_string(&opt.source)?;
    let assembly = PixelBenderShaderAssembly::new(&input, write);
    assembly.assemble()?;
    Ok(())
}

pub fn run_main(opt: Opt) -> Result<()> {
    let mut out: Box<dyn Write> = if let Some(output) = opt.output.as_ref().filter(|o| *o != "-") {
        Box::new(std::fs::File::create(output)?)
    } else {
        if !opt.disassemble && std::io::stdout().is_terminal() {
            return Err(PbasmError::BinaryDataInTerminalError);
        }
        Box::new(std::io::stdout())
    };

    if opt.disassemble {
        disassemble(opt, &mut out)?;
    } else {
        assemble(opt, &mut out)?;
    }

    Ok(())
}
