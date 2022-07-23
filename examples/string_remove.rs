fn main() {
  let mut s = String::from("test string remove method");
  println!("string s got {} chars", std::mem::size_of_val(s.as_str()));
  s.remove(0); // remove first char
  dbg!(s);
}