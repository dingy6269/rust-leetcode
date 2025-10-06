use std::collections::HashMap;

struct Solution {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coords {
  x: i32, // 1..=8
  y: i32 // 1..=8
}

// type Directions = HashMap<String, [i32; 2]>;
type RawDirection = [i32; 2];

struct Directions {
  map: HashMap<&'static str, RawDirection>
}


impl Directions {
  pub fn new() -> Self {
    let mut map = HashMap::new();

    map.insert("L", [-1, 0]);
    map.insert("R", [1, 0]);
    map.insert("U", [0, 1]);
    map.insert("D", [0, -1]);
    map.insert("LU", [-1, 1]);
    map.insert("LD", [-1, -1]);
    map.insert("RU", [1, 1]);
    map.insert("RD", [1, -1]);

    Self {
      map
    }
  }

  pub fn transmute(&self, direction: impl AsRef<str>) -> RawDirection {
    let dir =  direction.as_ref();

    *self.map.get(dir).unwrap_or_else(|| {
      panic!("Invalid direction: {}", dir);
    })
  }
}


impl Coords {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn update(&mut self, direction: RawDirection)  {
    self.x = self.x + direction[0];
    self.y = self.y + direction[1];
  }
}

// king, alone =>
// => moves to t
// from s to t (with min amount of moves)
//

// a8
// h1

// PREVIOSUYLLY!!!
// a ... h === y
// 1 ..8 == x

// NEW

// 1..8 == x
// 1..8 == y

// 1-1
// 8-8

// [1, 1]
// [8, 8]

impl Solution {
  pub fn recycle_board(initial: Coords, end: Coords) -> i32 {
    while initial.x != end.x && 
    initial.y != end.y {

    };
  }
}

fn main() {
  let result = Solution::recycle_board();

  println!("{:?}", result);
}
