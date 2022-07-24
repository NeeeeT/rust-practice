fn main() {
  let arr = [33_i32; 5];
  println!("{}", arr[arr.len() - 1]);
  let a: [i32; 5] = [1, 3, 5, 7, 9];
  println!("{}", a[a.len() - 1]);
  let a_slice: &[i32] = &a[1..3];
  println!("{}", a_slice[0]);
  println!("{}", a_slice[1]);
  assert_eq!(a_slice, &[3, 5]);
}