fn main() {
  let mut s = String::from("hello world");
  println!("{}", first_word(&s));
  let word = first_word(&s);
  // s.clear();
  println!("{}", word)
}

fn first_word(s: &String) -> &str {
  &s[..1]
}