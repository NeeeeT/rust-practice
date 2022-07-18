fn main() {
  // let reference_to_nothing = dangle();
  let reference_to_string = no_dangle();
  println!("{}", reference_to_string);
}

// fn dangle() -> &String {
//   let s = String::from("hello");
//   &s
// }

fn no_dangle() -> String{
  let s = String::from("hello no dangle");
  s
}