use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
  pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let corr = h as f64 / (piles.len() as f64);

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for (i, pile) in piles.iter().enumerate() {
      let cel = (*pile) as f64 / corr;

      min = min.min(cel.floor() as i32);
      max = max.max(cel.ceil() as i32);
    }

    let mut left = min.clone();
    let mut right = max.clone();

    let mut bmin: i32 = i32::MAX;

    while left <= right {
      let mid = (left + right) / 2;

      let mut bsum = 0;

      for pile in &piles {
        let div = *pile as f64 / mid as f64;
        let amount = div.ceil() as i32;

        bsum += amount;
      }

      if bsum == h {
        bmin = mid;
        break;
      }

      if bsum > h {
        left = mid + 1;
      }

      if bsum < h {
        right = mid - 1;
      }
    }

    // thats wierd here
    // alot between left and bmin
    for l in left..bmin {
      let mut bsum = 0;

      for pile in &piles {
        let div = *pile as f64 / l as f64;
        let amount = div.ceil() as i32;

        bsum += amount;
      }

      if bsum == h && l < bmin {
        bmin = l;
      }
    }

    bmin
  }
}

fn main() {
  let s1 = Solution::min_eating_speed(vec![3, 6, 7, 11], 8);
  let s2 = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
  let s3 = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6);

  println!("{:?}", s1);
  println!("{:?}", s2);
  println!("{:?}", s3);
}

// [3, 6, 7, 11]

// 3

// 4
// 2

// 4
// 3

// 4
// 4
// 3

// corr h / len

// 8 / 4 = 2

// 1.5
// 3
// 3.5
// 5.5

// 1..6

// =====

// 5 /5 = 1

// 30
// 11
// 23
// 4
// 20

// =============

// corr h / len

// 5 / 5 == 1

// 30 ?? 1 (mod)
// 30

// 11 ?? 1 (mod)

// corr h / len

// 6 / 5 == 1.25
//
