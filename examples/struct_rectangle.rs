#[derive(Debug)]
struct Rectangle {
  width: u64,
  height: u64,
}

fn main() {
  let rect = Rectangle {
    width: 120,
    height: 40,
  };
  println!("{:?}, {:?}", rect.width, rect.height);
}