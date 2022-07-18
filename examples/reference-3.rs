fn main() {
  let mut s = String::from("hello ref me");
  let r1 = &mut s;
  println!("{}", r1);
  let r2 = &mut s;
  println!("{}", r2);
}