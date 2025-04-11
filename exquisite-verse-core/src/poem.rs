use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};

use std::string::FromUtf8Error;
use base64::DecodeError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DeobfuscationError {
    Utf8(FromUtf8Error),
    Base64(DecodeError),
}

impl From<FromUtf8Error> for DeobfuscationError {
    fn from(err: FromUtf8Error) -> Self {
        DeobfuscationError::Utf8(err)
    }
}

impl From<DecodeError> for DeobfuscationError {
    fn from(err: DecodeError) -> Self {
        DeobfuscationError::Base64(err)
    }
}

fn base64_str_encode(input: &str) -> String {
    STANDARD.encode(input.as_bytes())
}

fn base64_str_decode(input: &str) -> Result<String, DeobfuscationError> {
    let decoded = STANDARD.decode(input)?;           // returns base64::DecodeError
    let parsed = String::from_utf8(decoded)?;       // returns FromUtf8Error
    Ok(parsed)
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Poem {
    lines: Vec<String>,
}

impl Poem {

    pub fn new(input: &str) -> Self {
        let lines = input.split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        Self {
            lines,
        }
    }

    pub fn from_semi_obfuscated(input: &str) -> Result<Self, DeobfuscationError> {
        let mut result: Vec<String> = Vec::new();
        let lines = input.split("\n").collect::<Vec<_>>();
        if let Some((last, elements)) = lines.split_last() {
            for line in elements {
                result.push(base64_str_decode(line)?);
            }
            result.push(last.to_string());
        }
        Ok(Self {
            lines: result,
        })
    }

    pub fn from_fully_obfuscated(input: &str) -> Result<Self, DeobfuscationError> {
        let mut result: Vec<String> = Vec::new();
        let lines = input.split("\n").collect::<Vec<_>>();
        for line in lines {
            result.push(base64_str_decode(line)?);
        }
        Ok(Self {
            lines: result,
        })
    }

    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn line_as_base64(&self, line_index: usize) -> String {
        base64_str_encode(&self.lines[line_index])
    }

    pub fn as_cleartext(&self) -> String {
        self.lines.join("\n")
    }

    pub fn as_semi_obfuscated(&self) -> String {
        if self.lines.is_empty() {
            return String::new();
        }
        
        let mut result = Vec::new();
        
        // Process all lines except the last one
        for line in &self.lines[..self.lines.len()-1] {
            result.push(base64_str_encode(line));
        }
        
        // Add the last line without encoding
        if let Some(last_line) = self.lines.last() {
            result.push(last_line.clone());
        }
        
        result.join("\n")
    }

    pub fn as_fully_obfuscated(&self) -> String {
        if self.lines.is_empty() {
            return String::new();
        }
        
        let mut result = Vec::new();
        
        // Process all lines except the last one
        for line in &self.lines {
            result.push(base64_str_encode(line));
        }
        
        result.join("\n")
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_poem() {
        let test_poem: Poem = Poem::new("Roses are red\nViolets are blue\nSugar is sweet\nAnd so are you.");
        println!("{}", test_poem.as_cleartext());
        println!("{}", test_poem.as_semi_obfuscated());
        println!("{}", test_poem.as_fully_obfuscated());
    }

    fn test_from_semi_obfuscated() {
        let semi_obfuscated = "Um9zZXMgYXJlIHJlZA==\nVmlvbGV0cyBhcmUgYmx1ZQ==\nU3VnYXIgaXMgc3dlZXQ=\nQW5kIHNvIGFyZSB5b3Uu";
        let fully_obfuscated = "Um9zZXMgYXJlIHJlZA==\nVmlvbGV0cyBhcmUgYmx1ZQ==\nU3VnYXIgaXMgc3dlZXQ=\nQW5kIHNvIGFyZSB5b3Uu";
        let cleartext = "Roses are red\nViolets are blue\nSugar is sweet\nAnd so are you.";

        assert_eq!(Poem::new(cleartext).as_semi_obfuscated(), semi_obfuscated);
        assert_eq!(Poem::new(cleartext).as_fully_obfuscated(), fully_obfuscated);
        assert_eq!(Poem::from_semi_obfuscated(semi_obfuscated).unwrap().as_cleartext(), cleartext);
        assert_eq!(Poem::from_fully_obfuscated(fully_obfuscated).unwrap().as_cleartext(), cleartext);
    }
}
