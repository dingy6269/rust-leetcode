fn main() {
  let x = [3, 4];
  let mut y = vec![5];
  // y, then x
  y.extend_from_slice(&x);

  println!("{:?}", y); // [5, 3, 4]
}
