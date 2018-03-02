mod parse;

#[macro_use]
extern crate nom;
extern crate serde;

//use serde::{Serializer, Deserializer};
use serde::Deserialize;
use parse::parse_string;
use nom::IResult::{Done, Incomplete};

pub type Result<T> = std::result::Result<T, ()>;
pub struct Deserializer<'de> {
    input: &'de [u8],
}

/*
impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input: input }
    }
}

pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T>
    where T: Deserialize<'a>
{
    let mut de = Deserializer::from_bytes(b);
    let t = T::deserialize(&mut de)?;
    Ok(t)
}
*/

impl<'de> Deserializer<'de> {
    fn parse_bool(&mut self) -> Result<bool> {
        match nom::be_u8(self.input) {
            Done(rest, result) => {
                self.input = &rest;
                Ok(result == 1)
            }
            Incomplete(_) => Err(()),
            nom::IResult::Error(_) => Err(()),
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        match nom::be_i32(self.input) {
            Done(rest, length) => {
                self.input = &rest[length as usize..];
                Ok(String::from_utf8_lossy(&rest[..length as usize]).to_string())
            }
            Incomplete(_) => Err(()),
            nom::IResult::Error(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_bool_works() {
        let mut thing = Deserializer { input: b"\x00\x01" };
        assert_eq!(thing.parse_bool().unwrap(), false);
        assert_eq!(thing.parse_bool().unwrap(), true);
    }

    #[test]
    fn parse_string_works() {
        let mut thing = Deserializer {
            input: b"\x00\x00\x00\x04asdf",
        };
        assert_eq!(thing.parse_string().unwrap(), String::from("asdf"));
    }

    #[test]
    fn parse_multiple_works() {
        let mut thing = Deserializer {
            input: b"\x00\x00\x00\x00\x0ai love you\x01",
        };
        assert_eq!(thing.parse_bool().unwrap(), false);
        assert_eq!(thing.parse_string().unwrap(), String::from("i love you"));
        assert_eq!(thing.parse_bool().unwrap(), true);
    }
}
