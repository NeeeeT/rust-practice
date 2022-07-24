fn main() {
  let string_append = String::from("hello ");
  let string_rust = String::from("rust");
  let mut res = string_append + &string_rust;
  res = res + "!!!";
  println!("{}", res);
}