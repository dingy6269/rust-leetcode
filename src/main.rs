#[derive(Debug, PartialEq, Eq)]
struct MinStack {
  vals: Vec<i32>,
  mins: Vec<i32>,
}

impl MinStack {
  fn new() -> Self {
    Self { vals: vec![], mins: vec![] }
  }

  fn push(&mut self, val: i32) {
    let m = self.mins.last().copied().map(|t| t.min(val)).unwrap_or(val);

    self.vals.push(val);
    self.mins.push(m);
  }

  fn pop(&mut self) {
    // because it has the SAME index there
    self.mins.pop();
    self.vals.pop();
  }

  fn top(&self) -> i32 {
    *self.vals.last().expect("no value error")
  }

  fn get_min(&self) -> i32 {
    *self.mins.last().unwrap()
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
