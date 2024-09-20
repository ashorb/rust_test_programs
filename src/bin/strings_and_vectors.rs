#![allow(dead_code)]

fn string_to_vector (s : &str) -> Vec<u8> {
  s.bytes ().collect::<Vec<_>> ()
}

fn vector_to_string (v : &[u8]) -> String {
  String::from_utf8_lossy (v).into_owned ()
}

fn main () {
  // TODO: try calling dbg!(string_to_vector(...)) and dbg!(vector_to_string(...))
  dbg!(string_to_vector("breakfast"));
  dbg!(vector_to_string(&[115]));
}