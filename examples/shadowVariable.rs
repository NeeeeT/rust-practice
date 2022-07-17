fn main() {
  let mut x: i32 = 300;
  x += 1;
  println!("x: {}", x);
  {
    let x = x * 3;
    println!("x: {}", x);
  }
  println!("x: {}", x);
}