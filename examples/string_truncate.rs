fn main() {
  let mut s = String::from("hello there!");
  s.truncate(5);
  println!("{}", s);
}