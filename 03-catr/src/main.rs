use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use simple_eyre::eyre::Result;

/// Rusty cat
#[derive(Debug, Parser)]
#[command(version)]
struct Config {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

fn main() -> Result<()> {
    simple_eyre::install()?;

    let config = Config::parse();

    for filename in config.files {
        let mut blank = 0;

        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                for (n, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", n + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            blank += 1;
                            println!()
                        } else {
                            println!("{:>6}\t{}", n + 1 - blank, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
