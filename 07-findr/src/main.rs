use clap::{Parser, ValueEnum};
use regex::Regex;
use simple_eyre::eyre::Result;
use walkdir::{DirEntry, WalkDir};

/// Rusty find
#[derive(Debug, Parser)]
#[command(version)]
struct Config {
    /// Name
    #[arg(short, long, num_args=1..)]
    name: Vec<Regex>,

    /// Entry Type
    #[arg(short, long, value_enum, num_args=1..)]
    r#type: Vec<EntryType>,

    /// Search paths
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    #[value(name = "d")]
    Dir,
    #[value(name = "f")]
    File,
    #[value(name = "l")]
    Link,
}

fn main() -> Result<()> {
    simple_eyre::install()?;
    let config = Config::parse();

    let filter_by_type = |entry: &DirEntry| -> bool {
        config.r#type.is_empty()
            || config.r#type.iter().any(|t| match t {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let filter_by_name = |entry: &DirEntry| -> bool {
        config.name.is_empty()
            || config
                .name
                .iter()
                .any(|n| n.is_match(entry.file_name().to_string_lossy().as_ref()))
    };

    for path in config.paths {
        let entries: Vec<_> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(filter_by_type)
            .filter(filter_by_name)
            .map(|entry| entry.path().display().to_string())
            .collect();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
