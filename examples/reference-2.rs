fn main() {
  let mut s = String::from("change me");
  change(&mut s);
  println!("{}", s);
}

fn change(some_string: &mut String) {
  some_string.push_str(" oh changed");
}