fn main() {
  let one = [1, 2, 3];
  let two: [u8; 3] = [4, 5, 6];
  let blank1 = [0; 3];
  let blank2: [u8; 3] = [0; 3];
  let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
  for i in arrays.iter() {
    println!("{:?}:", i);
    for n in i.iter() {
      println!("\t{} + 10 = {}", n, n + 10);
    }
    let mut sum = 0;
    for j in i {
      sum += j;
    }
    println!("Sum: {:?}", sum);
  }
}