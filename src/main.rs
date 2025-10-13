// car fleet;

// n cars (from mile 0) travellign to target

// pos[]
// speed[]

// cannot catch up

// speed of car fleet = minimum speed of any car

// (if a car to traget it still a fleet)

// target 12
// position 10, 8, 0, 5, 3
// speed 2, 4, 1, 1, 3

// 3

// 10 + 2
// 8  + 4
// ==>> 12

// 5 + 1
// 3 + 3
// => 6 (1)

// -----
// target 100
// 0, 2, 4
// 4, 2, 1
// =>
// 4, 4, 5
// 8, 6, 6

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
  pub fn car_fleet(
    target: i32,
    position: Vec<i32>,
    speed: Vec<i32>,
  ) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (idx, pos) in position.iter().enumerate() {
      let npos = pos + speed[idx];

      println!("npos {:?}", npos);

      map
        .entry(npos)
        .and_modify(|v| v.push(idx as i32))
        .or_insert(vec![idx as i32]);
      // .or_default();
    }

    println!("{:#?}", map);

    println!("{:#?}", map.len());

    return 0;
  }
}

fn main() {
  // Solution::car_fleet(
  //   12,
  //   vec![10, 8, 0, 5, 3],
  //   vec![2, 4, 1, 1, 3],
  // );

  Solution::car_fleet(
    100,
    vec![0, 2, 4],
    vec![4, 2, 1],
  );
}
