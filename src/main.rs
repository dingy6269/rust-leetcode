
#[derive(Debug, PartialEq, Eq)]
struct MinStack {
  vals: Vec<i32>,
  mins: Vec<i32>
}

impl MinStack {
  fn new() -> Self {
    Self {
      vals: vec![],
      mins: vec![]
    }
  }

  fn push(&mut self, val: i32) {
    self.vals.push(val);
    self.mins.
  }

  fn pop(&mut self) {
    self.vals.pop();
  }

  fn top(&self) -> i32 {
    *self.vals.last().expect("no value error")
    }

  fn get_min(&self) -> i32 {
    let value = self.vals.iter().min();

    *value.expect("no value error")
  }
}


fn main() {
  let mut stack = MinStack::new();

  stack.push(-2);
  stack.push(0);
  stack.push(-3);

  let min = stack.get_min();

  println!("{:?}", min);

  stack.pop();
  
  println!("{:?}", stack.top());
  println!("{:?}", stack.get_min())
}