extern crate nom;

use nom::be_i32;

/* This is my compiles-without-excess-compiler-trash
 * playground file. I noodle on ideas here.
 * In the future, this may become the actual parsing file.
 * It remains to be seen
 */


named!(pub parse_string<&[u8], String>,
  do_parse!(
    len: be_i32 >>
    data: take!(len) >>
    (String::from_utf8_lossy(data).to_string())
  )
);

named!(pub parse_bool<&[u8], bool>,
  do_parse!(
    byte: take!(1) >>
    (byte[0] == 1)
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

  #[test]
  fn simple_bool_parses() {
    let a = b"\x00";
    let b = b"\x01";
    assert_eq!(parse_bool(a).to_result().unwrap(), false);
    assert_eq!(parse_bool(b).to_result().unwrap(), true);
  }
}