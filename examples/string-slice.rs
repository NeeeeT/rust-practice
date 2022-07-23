fn main() {
  let s = String::from("hello world😂");
  println!("{}", &s[3..5]);
  println!("{}", &s[3..s.len()]);
}