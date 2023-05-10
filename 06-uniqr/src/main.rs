use clap::Parser;
use simple_eyre::eyre::{eyre, Result};

use std::io::{BufRead, Write};

/// Rusty uniq
#[derive(Debug, Parser)]
#[command(version)]
struct Config {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

fn main() -> Result<()> {
    simple_eyre::install()?;

    let config = Config::parse();

    let mut file = util::open(&config.in_file).map_err(|e| eyre!("{}: {}", config.in_file, e))?;
    let mut count: usize = 0;
    let mut line = String::new();
    let mut prev_line = String::new();

    let mut out = util::out(&config.out_file.unwrap_or(String::from("-")))?;

    let mut print = |count: usize, text: &str| -> Result<()> {
        if count > 0 {
            if config.count {
                write!(out, "{:>4} {}", count, text)?;
            } else {
                write!(out, "{}", text)?;
            }
        }
        Ok(())
    };
    loop {
        let read = file.read_line(&mut line)?;
        if read == 0 {
            break;
        }
        if line.trim_end() != prev_line.trim_end() {
            print(count, &prev_line)?;
            prev_line = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }
    print(count, &prev_line)?;

    Ok(())
}
