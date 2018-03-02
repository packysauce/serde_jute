#![allow(unused_imports)]

mod parse;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate serde;

mod errors {
    error_chain! {
        foreign_links {
            Str(::std::str::Utf8Error);
        }
        errors {
            IsNegative
            Incomplete
        }
    }
}

//use serde::{Serializer, Deserializer};
use errors::*;
use serde::Deserialize;
use parse::parse_buffer;
use nom::IResult::{Done, Incomplete};

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
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_buffer(&mut self) -> Result<&'de [u8]> {
        match parse::parse_buffer(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            }
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        match self.parse_buffer() {
            Ok(bytes) => Ok(std::str::from_utf8(bytes)?),
            Err(e) => Err(Error::with_chain(e, "UTF-8 decode failed"))
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
    fn parse_string_too_little_data() {
        let mut thing = Deserializer {
            input: b"\x00\x00\x00\x04as",
        };
        let e = thing.parse_string();
        assert_eq!(e.is_err(), true);
    }

    #[test]
    fn parse_buffer_works() {
        let mut thing = Deserializer {
            input: b"\x00\x00\x00\x04\x01\x02\x03\x04",
        };
        assert_eq!(thing.parse_buffer().unwrap(), [1, 2, 3, 4]);
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
