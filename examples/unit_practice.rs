fn main() {
  for c in "台灣人".chars() {
    println!("{}", c);
  }
  for c in "台灣人".bytes() {
    println!("{}", c);
  }
}