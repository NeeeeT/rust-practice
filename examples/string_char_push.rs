fn main() {
  let mut s = String::from("new world");
  s.push('A');
  println!("{}", s);
  s.push_str("AAA");
  println!("{}", s);
}