use std::collections::HashMap;

struct Solution {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coords {
  x: i32, // 1..=8
  y: i32 // 1..=8
}

// type Directions = HashMap<String, [i32; 2]>;


struct Directions {
  map: HashMap<String, [i32; 2]>
}

impl Directions {
  pub fn new() {

    let map = HashMap::new();

    map.insert("L", [-1, 0]);
    map.insert("R", [1, 0]);
    map.insert("U", [0, 1]);
    map.insert("D", [0, -1]);
    map.insert("LU", [-1, 1]);
    map.insert("LD", [-1, -1]);
    map.insert("RU", [1, 1]);
    map.insert("RD", [1, -1]);

    Sefl {
      map
    }
  } 
}

const DIRECTIONS: [&'static str; 8] = [
  "L",
  "R",
  "U",
  "D",
  "LU",
  "LD",
  "RU",
  "RD"
];

// const Map= {

// }

impl Coords {
  pub fn new(x: i32, y: i32) -> Self {


    Self { x, y, map }
  }

  pub fn update(dir: String)  {

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
