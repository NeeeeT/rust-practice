fn main() {
  let x = (-42.0_f32).sqrt();
  println!("{}", x);
  if x.is_nan() {
    println!("未定義的數學行為");
  }
}