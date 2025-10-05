use std::rc::Rc;

pub fn main() {
  let a = Rc::new(5);
  let b = a.clone();

  println!("{}, {}", a, b);
}
