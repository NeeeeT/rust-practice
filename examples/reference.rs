fn main() {
  let x = 5;
  let y = &x;
  println!("{}, {}", x, y);
  println!("{}, {}", &x, *y);
}