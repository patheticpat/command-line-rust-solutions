use std::io::BufRead;
use std::ops::Add;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FileInfo {
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
}

impl Add for FileInfo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num_lines: self.num_lines + rhs.num_lines,
            num_words: self.num_words + rhs.num_words,
            num_bytes: self.num_bytes + rhs.num_bytes,
            num_chars: self.num_chars + rhs.num_chars,
        }
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo, std::io::Error> {
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_words = 0;
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes;
        num_chars += line.len();
        num_words += line.split_whitespace().count();
        line.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_bytes,
        num_chars,
        num_words,
    })
}

pub fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(537, true), "     537");
    }
}
