use std::collections::HashMap;

struct Solution {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coords {
  x: i32, // 1..=8
  y: i32, // 1..=8
}




impl Coords {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn update(&mut self, direction: RawDirection) {
    self.x = self.x + direction[0];
    self.y = self.y + direction[1];
  }

  pub fn from_square<S: AsRef<str>>(s_: S) -> Self {
    let s = s_.as_ref();

    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    const x_transmute: &'static str = "abcdefgh";
    const y_transmute: &'static str = "87654321";

    for (i ,c) in x_transmute.chars().enumerate() {
      x_map.insert(c, (i + 1) as i32);
    }

    for (i, c) in y_transmute.chars().enumerate() {
      y_map.insert(c, (i + 1) as i32);
    };

    // a8
    // h1

    // ['a', '8']
    let chars: Vec<char> = s.chars().collect();
    let x = x_map.get(&chars[0]).unwrap();
    let y = y_map.get(&chars[1], ).unwrap();

    Self::new(x.clone(), y.clone())
  }
}

// type Directions = HashMap<String, [i32; 2]>;
type RawDirection = [i32; 2];

struct Directions {
  map: HashMap<&'static str, RawDirection>,
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

    Self { map }
  }

  pub fn transmute(
    &self,
    direction: impl AsRef<str>,
  ) -> RawDirection {
    let dir = direction.as_ref();

    *self.map.get(dir).unwrap_or_else(|| {
      panic!("Invalid direction: {}", dir);
    })
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
  pub fn king_moves(s: Coords, t: Coords) -> () {
    // so we find deltas
    let mut dx = t.x - s.x;
    let mut dy = t.y - s.y;
    let mut moves = Vec::new();

    // dx, dy
    while dx != 0 || dy != 0 {
      let mut mv = String::with_capacity(2);

      if dx > 0 {
        mv.push('R');
        dx -= 1;
      } else if dx < 0 {
        mv.push('L');
        dx += 1;
      }

      // -8 => -7
      if dy > 0 {
        mv.push('D');
        dy -= 1;
      } else if dy < 0 {
        mv.push('U');
        dy += 1;
      }

      moves.push(mv);
    }

    println!("{}", moves.len());

    for mv in moves {
      println!("{}", mv);
    }

    // so int that case
    // instead of comparing intiail and end
    // we can create "delta"
    // and allocate the futher stuff
    // to the delta

    // while initial.x != end.x && initial.y != end.y {}
  }
}

// dx = tx - sx

// 8..1 = y
// a..h = x


// a8
// h1

fn main() {
  let start = Coords::from_square("a8");
  let finish = Coords::from_square("h1");

  let result = Solution::king_moves(start, finish);
}
