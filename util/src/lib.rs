//! Collection of utility functions
//!
//! This crate contains a growing number of utility functions to be used in the
//! solutions to [Command-Line Rust by Ken Youens-Clark](https://github.com/kyclark/command-line-rust.git).
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Result, Write},
};

/// Open a file for [`BufRead`]ing
///
/// `filename` can either be:
///
/// - a path to a file
/// - the special value `"-"`, which stands for [`io::Stdin`]
///
/// Returns either a [`Box<dyn std::io::BufRead>`] in case of success, or a [`std::io::Error`] otherwise.
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/// Open a file for [`Write`]ing
///
/// `filename` can either be:
///
/// - a path to a file
/// - the special value `"-"`, which stands for [`io::Stdout`]
///
/// Returns either a [`Box<dyn std::io::Write>`] in case of success, or a [`std::io::Error`] otherwise.
pub fn out(filename: &str) -> Result<Box<dyn Write>> {
    match filename {
        "-" => Ok(Box::new(io::stdout())),
        _ => Ok(Box::new(File::create(filename)?)),
    }
}
