use clap::Parser;
use simple_eyre::eyre::Result;

/// Rusty wc
#[derive(Debug, Parser)]
#[command(version)]
struct Config {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show byte count
    #[arg(short = 'c', long, conflicts_with = "chars")]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long)]
    chars: bool,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,
}
fn main() -> Result<()> {
    simple_eyre::install()?;

    let mut config = Config::parse();

    if [config.lines, config.bytes, config.chars, config.words]
        .iter()
        .all(|v| !v)
    {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }

    let mut total: wcr::FileInfo = Default::default();
    let mut count = 0usize;

    for filename in &config.files {
        match util::open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                count += 1;
                let info = wcr::count(file)?;
                total = total + info;
                print_file_info(
                    &info,
                    config.lines,
                    config.words,
                    config.bytes,
                    config.chars,
                    if filename == "-" { "" } else { filename },
                );
            }
        }
    }

    if count > 1 {
        // Output total
        print_file_info(
            &total,
            config.lines,
            config.words,
            config.bytes,
            config.chars,
            "total",
        );
    }
    Ok(())
}

fn print_file_info(
    info: &wcr::FileInfo,
    show_lines: bool,
    show_words: bool,
    show_bytes: bool,
    show_chars: bool,
    title: &str,
) {
    println!(
        "{}{}{}{}{}{}",
        wcr::format_field(info.num_lines, show_lines),
        wcr::format_field(info.num_words, show_words),
        wcr::format_field(info.num_bytes, show_bytes),
        wcr::format_field(info.num_chars, show_chars),
        if title.is_empty() { "" } else { " " },
        title
    );
}
