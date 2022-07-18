fn main() {
  // Without type defining is incorrect.
  // let guess = "42".parse().expect("Not a number");
  let guess: f32 = "42".parse().expect("Not a number");
  println!("{}", guess);
}