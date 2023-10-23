mod awacpu;
mod errors;

use awassembler::{awassemble, print_awatisms};
use clap::{Parser, Subcommand};
use errors::AwawaError;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::ExitCode,
};

use crate::awacpu::AwaCPU;
mod awassembler;

#[derive(Parser)]
#[command(author, version, about)]
/// Awatistic AWA5.0 Interpreter
///
/// Based on the AWA5.0 language specification by Temp-Tempai: https://github.com/TempTempai/AWA5.0/
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Awatalk
    Run {
        /// File containing Awatalk to run (defaults to stdin)
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Verbosity of output, can be specified up to three times
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// Disawassemble Awatalk
    Disawassemble {
        /// File containing Awatalk to disawassemble (defaults to stdin)
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
    /// Awassemble Awatisms
    Awassemble {
        /// File containing Awatisms to awassemble (defaults to stdin)
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, verbose } => match file_or_stdin_to_string(file) {
            Err(e) => {
                eprintln!("Error reading awawa input: {e}");
                ExitCode::from(3)
            }
            Ok(awa) => run(awa.as_str(), verbose),
        },
        Commands::Disawassemble { file } => match file_or_stdin_to_string(file) {
            Err(e) => {
                eprintln!("Error reading awawa input: {e}");
                ExitCode::from(6)
            }
            Ok(awa) => disawassemble(awa.as_str()),
        },
        Commands::Awassemble { file } => {
            let reader = match file_or_stdin(file) {
                Err(e) => {
                    eprintln!("Error reading awawa input: {e}");
                    return ExitCode::from(1);
                }
                Ok(r) => r,
            };

            let mut awatisms = vec![];
            match awassemble(reader, &mut awatisms) {
                Err(e) => {
                    for (i, awa) in awatisms.iter().enumerate() {
                        eprintln!("[{i}] {awa}");
                    }
                    eprintln!("Error awassembling instruction: {e}");
                    return ExitCode::from(1);
                }
                _ => (),
            }

            let mut out = String::new();
            match print_awatisms(awatisms, &mut out) {
                Err(e) => {
                    eprintln!("Error printing awatisms: {e}");
                    return ExitCode::from(2);
                }
                _ => (),
            }
            println!("{out}");
            ExitCode::from(0)
        }
    }
}

fn file_or_stdin_to_string(file: Option<PathBuf>) -> Result<String, std::io::Error> {
    match file {
        Some(file) => {
            if file.as_os_str() == "-" {
                io::read_to_string(io::stdin())
            } else {
                fs::read_to_string(file)
            }
        }
        None => io::read_to_string(io::stdin()),
    }
}

fn file_or_stdin(file: Option<PathBuf>) -> Result<Box<dyn BufRead>, std::io::Error> {
    let reader: Box<dyn std::io::BufRead> = match file {
        Some(file) => {
            if file.as_os_str() == "-" {
                Box::new(BufReader::new(io::stdin()))
            } else {
                match File::open(file) {
                    Ok(f) => Box::new(BufReader::new(f)),
                    Err(e) => return Err(e),
                }
            }
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    return Ok(reader);
}

fn disawassemble(awa: &str) -> ExitCode {
    let mut cpu = AwaCPU::new(awa.chars(), 0);
    match cpu.disawassemble() {
        Err(e) => {
            eprintln!("Error disawassembling instruction: {e}");
            return ExitCode::from(7);
        }
        _ => return ExitCode::from(0),
    }
}

fn run(awa: &str, verbose: u8) -> ExitCode {
    let mut cpu = AwaCPU::new(awa.chars(), verbose);
    match cpu.load_program() {
        Err(e) => {
            eprintln!("Failed to load program:");
            for (i, awatism) in cpu.get_program().iter().enumerate() {
                eprintln!("[{i}] {awatism}");
            }
            eprintln!(
                "Error parsing instruction {0}: {e}",
                cpu.get_program().len()
            );
            return ExitCode::from(4);
        }
        _ => (),
    }
    match cpu.run() {
        Err(AwawaError::EndOfProgramError()) => {
            if verbose > 0 {
                println!("Program ended.");
            }
            return ExitCode::from(0);
        }
        Err(e) => {
            eprintln!("Error executing instruction {0}: {e}", cpu.get_ip());
            eprintln!("Bubble Abyss:");
            eprintln!("{0}", cpu.get_bubble_abyss());
            eprintln!("Program:");
            for (i, awatism) in cpu.get_program().iter().enumerate() {
                if i == cpu.get_ip() {
                    eprint!("->");
                } else {
                    eprint!("  ");
                }
                eprintln!("[{i}] {awatism}");
            }
            return ExitCode::from(5);
        }
        _ => return ExitCode::from(0),
    }
}
