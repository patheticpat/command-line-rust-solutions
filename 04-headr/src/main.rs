use clap::{value_parser, Parser};
use simple_eyre::eyre::Result;
use std::io::{BufRead, Read};

/// Rusty head
#[derive(Parser, Debug)]
#[command(version)]
struct Config {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        value_parser = value_parser!(u64).range(1..),
        default_value_t = 10,
        conflicts_with = "bytes"
    )]
    lines: u64,

    #[arg(short = 'c', long, value_parser = value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}

fn main() -> Result<()> {
    simple_eyre::install()?;

    let config = Config::parse();
    let show_headers = config.files.len() > 1;
    for (file_num, filename) in config.files.iter().enumerate() {
        match util::open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Some(bytes) = config.bytes {
                    if show_headers {
                        println!(
                            "{}==> {} <==",
                            if file_num > 0 { "\n" } else { "" },
                            filename
                        );
                    }
                    let mut file = file.take(bytes);
                    let mut buf = Vec::with_capacity(bytes as usize);
                    file.read_to_end(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf));
                } else {
                    if show_headers {
                        println!(
                            "{}==> {} <==",
                            if file_num > 0 { "\n" } else { "" },
                            filename
                        );
                    }
                    // splitting on '\n' will leave a possible '\r' in place, whereas iterating over
                    // .lines() will remove both.
                    for line in file.split(b'\n').take(config.lines as usize) {
                        let line = line?;
                        let line = String::from_utf8_lossy(&line);
                        println!("{}", line);
                    }
                }
            }
        }
    }

    Ok(())
}
