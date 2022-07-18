fn main() {
  let s1 = gives_ownership();
  println!("{}", s1);
  let mut  s2 = String::new();
  s2.push_str("Hello its s2");
  println!("{}", s2);
  let s3 = takes_and_gives_back(s2);
  println!("{}", s3);
  // print s2 again then you'll get an error msg.
  // println!("{}", s2);
}

fn gives_ownership() -> String {
  let some_string = String::from("Hello its s1");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}
