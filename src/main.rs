use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use transliterator::Mode;

/// Transliterate Serbian Cyrillic and Latin scripts.
///
/// Reads from the given FILES (concatenated, in order) or from stdin when no
/// file is provided. Writes the result to stdout, or to the file given by
/// `--out`.
#[derive(Debug, Parser)]
#[command(name = "transliterate", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Output Serbian Latin (with diacritics). Cyrillic is converted; Latin
    /// passes through unchanged.
    Lat(Io),
    /// Output diacritic-free Latin (URL-safe). Handles Cyrillic and
    /// diacritic Latin in the same input.
    CutLat(Io),
    /// Output Serbian Cyrillic. Latin (with diacritics) is converted;
    /// Cyrillic passes through unchanged.
    Cir(Io),
}

impl Cmd {
    fn io(&self) -> &Io {
        match self {
            Cmd::Lat(io) | Cmd::CutLat(io) | Cmd::Cir(io) => io,
        }
    }

    fn apply(&self, input: &str) -> String {
        match self {
            Cmd::Lat(_) => Mode::CirToLat.apply(input),
            // Run Cyrillic→cut-Latin first, then strip any remaining
            // diacritics from native Latin text. Either pass is a no-op on
            // text that doesn't contain its source script.
            Cmd::CutLat(_) => {
                let pass1 = Mode::CirToCutLat.apply(input);
                Mode::LatToCutLat.apply(&pass1)
            }
            Cmd::Cir(_) => Mode::LatToCir.apply(input),
        }
    }
}

#[derive(Debug, clap::Args)]
struct Io {
    /// Input files. Reads stdin if omitted (use shell redirection: `< file`).
    files: Vec<PathBuf>,

    /// Write output to this file instead of stdout.
    #[arg(short, long = "out", value_name = "PATH")]
    out: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("transliterate: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> io::Result<()> {
    let io_args = cli.command.io();
    let input = read_input(&io_args.files)?;
    let output = cli.command.apply(&input);

    match &io_args.out {
        Some(path) => {
            let f = File::create(path)?;
            let mut w = BufWriter::new(f);
            w.write_all(output.as_bytes())?;
            w.flush()?;
        }
        None => {
            let stdout = io::stdout();
            let mut w = stdout.lock();
            w.write_all(output.as_bytes())?;
        }
    }
    Ok(())
}

fn read_input(files: &[PathBuf]) -> io::Result<String> {
    if files.is_empty() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        return Ok(buf);
    }

    let mut combined = String::new();
    for path in files {
        let mut f = File::open(path).map_err(|e| {
            io::Error::new(e.kind(), format!("{}: {e}", path.display()))
        })?;
        f.read_to_string(&mut combined)?;
    }
    Ok(combined)
}
