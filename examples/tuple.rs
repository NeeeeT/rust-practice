fn main() {
  let tup = (500, 6.4, 1);

  let (_x, _y, _z) = tup;

  println!("The value of y is: {}", _y);
  println!("{}", tup.2);

  let string_s = String::from("Hello world");
  let (s2, len) = calculate_length(string_s);
  println!("{}, {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}