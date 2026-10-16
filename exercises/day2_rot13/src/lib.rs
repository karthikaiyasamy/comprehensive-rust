use std::io::{self, Read};

pub struct Rot13<R: Read> {
    pub input: R,
}

impl<R: Read> Rot13<R> {
    pub fn new(input: R) -> Self {
        Rot13 { input }
    }
}

fn rot13_byte(b: u8) -> u8 {
    match b {
        b'a'..=b'm' | b'A'..=b'M' => b + 13,
        b'n'..=b'z' | b'N'..=b'Z' => b - 13,
        other => other,
    }
}

impl<R: Read> Read for Rot13<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.input.read(buf)?;
        for b in &mut buf[..bytes_read] {
            *b = rot13_byte(*b);
        }
        Ok(bytes_read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13() {
        let source = "Hello, World!";
        let mut decoder = Rot13::new(source.as_bytes());
        let mut result = String::new();
        decoder.read_to_string(&mut result).unwrap();
        assert_eq!(result, "Uryyb, Jbeyq!");

        let mut roundtrip = Rot13::new(result.as_bytes());
        let mut restored = String::new();
        roundtrip.read_to_string(&mut restored).unwrap();
        assert_eq!(restored, "Hello, World!");
    }
}
