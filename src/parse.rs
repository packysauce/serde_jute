extern crate nom;

use nom::be_i32;

named!(pub parse_string<&[u8], String>,
    do_parse!(
        len: be_i32 >>
        data: take!(len) >>
        (String::from_utf8_lossy(data).to_string())
    )
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_string_parses() {
    let a = b"\x00\x00\x00\x04asdf";
    assert_eq!(parse_string(a).to_result().unwrap(), String::from("asdf"));
  }
}