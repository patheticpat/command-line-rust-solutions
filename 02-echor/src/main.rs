use clap::Parser;

/// Rust echo
#[derive(Parser)]
#[command(about, version)]
struct Args {
    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,

    /// Input Text
    #[arg(required = true)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let ending = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), ending);
}
