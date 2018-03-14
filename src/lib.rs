#![allow(unused_imports)]

mod parse;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate serde;

use std::fmt::Display;

use errors::*;
use serde::Deserialize;
use parse::parse_buffer;
use nom::IResult::{Done, Incomplete};

mod errors {
    error_chain! {
        foreign_links {
            Str(::std::str::Utf8Error);
        }
        errors {
            IsNegative
            Incomplete
            OnlySigned
            InvalidJuteType(what: String)
            Other(message: String)
        }
    }
}

//use serde::{Serializer, Deserializer};

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
            // this is straight incorrect
            Err(e) => Err(Error::with_chain(e, "UTF-8 decode failed"))
        }
    }

    fn parse_long(&mut self) -> Result<i64> {
        match nom::be_i64(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            },
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_i8(&mut self) -> Result<i8> {
        match nom::be_i8(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            },
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_i16(&mut self) -> Result<i16> {
        match nom::be_i16(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            },
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_i32(&mut self) -> Result<i32> {
        match nom::be_i32(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            },
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }

    fn parse_i64(&mut self) -> Result<i64> {
        match nom::be_i64(self.input) {
            Done(rest, data) => {
                self.input = &rest;
                Ok(data)
            },
            Incomplete(_) => Err(ErrorKind::Incomplete.into()),
            nom::IResult::Error(e) => bail!(e.description())
        }
    }
}

impl serde::de::Error for Error {

    fn custom<T: Display>(msg: T) -> Self{
        ErrorKind::Other(msg.to_string()).into()
    }

}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ErrorKind::Other("deserialize_any not supported".to_string()).into())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ErrorKind::OnlySigned.into())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ErrorKind::OnlySigned.into())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ErrorKind::OnlySigned.into())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ErrorKind::OnlySigned.into())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }
    
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("f32".to_string()).into())
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("f64".to_string()).into())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("char".to_string()).into())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("str".to_string()).into())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("String".to_string()).into())
    }
    
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        visitor.visit_bytes(self.parse_buffer()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        visitor.visit_byte_buf(Vec::from(self.parse_buffer()?))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("Option".to_string()).into())
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("Seq (for now)".to_string()).into())
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("Tuple (for now)".to_string()).into())
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        Err(ErrorKind::InvalidJuteType("tuple struct".to_string()).into())
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
            input: b"\x00\x00\x00\x00\x0ai love you\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01",
        };
        assert_eq!(thing.parse_bool().unwrap(), false);
        assert_eq!(thing.parse_string().unwrap(), String::from("i love you"));
        assert_eq!(thing.parse_bool().unwrap(), true);
        assert_eq!(thing.parse_long().unwrap(), 1);
        assert_eq!(thing.parse_int().unwrap(), 1)
    }
}
