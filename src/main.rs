use std::{collections::HashMap, panic::resume_unwind};

struct Solution {}

pub fn is_odd(x: i32) -> bool {
  x % 2 == 0
}

impl Solution {
  pub fn codeforces_4a(w: i32) -> bool {
    if (!is_odd(w)) {
      return false;
    };

    // 1

    let ileft = 1;
    // 7
    let iright = w - 1;

    let mut left = ileft;
    // 7
    let mut right = iright;

    let mut results: Vec<(i32, i32)> = Vec::new();

    // 4 4
    while (left <= right) {
      if (!is_odd(left)) {
        left += 1
      };

      if (!is_odd(right)) {
        right -= 1
      };

      // 2 6 => 3 5
      if (is_odd(left) && is_odd(right)) {
        results.push((left, right));

        left += 1;
        right -=1;
      }
    }


    let mut ileft = 1;
    // 7
    let mut iright = w - 1;

    println!("l {:?}", left);
    println!("r {:?}", right);


    println!("il {:?}", ileft);
    println!("ir {:?}", iright);

    while (left < iright && right > ileft) {
      if (!is_odd(left)) {
        left += 1
      };

      if (!is_odd(right)) {
        right -= 1
      };

      // 2 6 => 3 5
      if (is_odd(left) && is_odd(right)) {
        results.push((left, right));

        left += 1;
        right -=1;
      }
    }
    // 4 4 => 5 3k
    // while (left )

    println!("{:?}", results);

    results.len() > 0
  }
}

// two pointers => two times
// without colliding

// single watermln
// two instances

// biggest (max)
// computed => "w"

// tried to divide (between 2)
// constrains:
// 1. % 2 == 0 (can be diff size) (yeah and >0 each)

// => bool

// so it can be 4 and 4
// 2 and 6
// 6 and 2
//

// (9) the initial value sohuld be % 2 == 0
// (10)
/// 2 8
/// 4 6
/// 6 4
/// 8 2
///
/// 2
/// 1 1

fn main() {

    let _ = Solution::codeforces_4a(8);
}

#[cfg(test)]
mod tests {
  use crate::Solution;

  #[test]
  pub fn case_1() {
    let result = Solution::codeforces_4a(8);
  }
}
