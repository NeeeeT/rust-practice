enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}
fn main() {
  let m1 = Message::Quit;
  let m2 = Message::Move{x: 33, y: 34};
  let m3 = Message::ChangeColor(255, 255, 0);
}