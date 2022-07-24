#[derive(Debug)]
enum Option<T> {
  Some(T),
  None,
}

fn main () {
  let x = Option::Some(1);
  let y = Option::Some("A string");
  let z: Option<i32> = Option::None;
  println!("{:?}, {:?}, {:?}", x, y, z);
}