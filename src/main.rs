
#[derive(Debug, PartialEq, Eq)]
struct MinStack {
  values: Vec<i32>,
  head: Option<i32>,
  previous: Option<i32>
}

impl MinStack {
  fn new() -> Self {
    Self {
      values: vec![],
      head: None,
      previous: None
    }
  }

  fn push(&mut self, val: i32) {
    self.values.push(val);
    
    self.previous = self.head;
    self.head = Some(val);
  }

  fn pop(&mut self) {
    self.values.pop();
    self.head = self.previous;
    self.previous = None;
  }

  fn top(&self) -> i32 {
    self.head.unwrap_or_else(|| {
      panic!("shoudn't happen here")
    })
  }

  fn get_min(&self) -> i32 {
    let value = self.values.iter().min();

    value.unwrap_or_else(|| {
      panic!("Shouldn't happen")
    }).clone()
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