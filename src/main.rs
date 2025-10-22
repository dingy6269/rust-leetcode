// add - 0 traffic
// delete - 0 traffic
// send (al all and the sender) - (Lb to each where l is length)

// amount of traffic?

// 5
// 4 (including kate)

// to long - length > than 10 symbols
// too

use std::collections::HashSet;
use std::io::{self, BufRead};

//


// O(n * k) - STL
// O(n) - Sliding window

// sliding window

// Window 1: [1, 2, 6]
// Calculate: 1 + 2 + 6 = 9  (3 additions)
//
// Window 2: [2, 6, 1]
// Calculate: 2 + 6 + 1 = 9  (3 additions)
// ↑   ↑
// These are REPEATED from Window 1!
//

fn s_solution(mut k: i32, mut vec: Vec<i32>) -> i32 {
  // k = 3
  let k = k as usize;

  // k = size of each window (number of consecutive elements to sum)
  

  // first window
  let first_window = &vec[0..k];
  let mut sum: i32 = first_window.iter().sum(); // recalculates sum here

  // first sum (not i32::MAX)
  let mut min = sum;
  let mut midx = 0;

  // because we took 0
  // 1, 4
  for i in 1..=(vec.len() - k) {
    // Window 3: [6, 1, 1]
    // Calculate: 9 - 2 + 1 = 8  (1 subtraction + 1 addition)

    // re-calculate new sum
    // REMOVE OLD, ADD NEW (SLIDING WINDOW)
    sum = sum - vec[i - 1] + vec[i + k - 1];

    if sum < min {
      min = sum;
      midx = i;
    }
  }

  (midx + 1) as i32 // 1-based indexing
}


fn solution(mut k: i32, mut vec: Vec<i32>) -> i32 {
  let mut min = i32::MAX;
  let mut midx = 0;

  for i in 0..vec.len() {

    // genius check,
    if i + (k as usize) <= vec.len() {
      // repetition happens here
      // so sliding window fixes it
      let sub = &vec[i..i + (k as usize)];

      dbg!(&sub);

      let sum = sub.iter().sum::<i32>();

      dbg!(&sum);

      if sum < min {
        min = sum;
        // 1-based indexing
        midx = (i + 1)
      }
    }

    // 1, 2, 6
  };

  midx as i32
}

//
// fn solution(mut T: i32, mut vec: Vec<i32>) -> i32 {
//   // if vector IS empty??
//   if vec.is_empty() {
//     return 0;
//   }
//
//   let mut r = 0;
//
//   for start in 0..vec.len() {
//     let mut n = 0;
//     let mut t = T.clone();
//
//     for i in (start as usize)..vec.len() {
//       if t < vec[i] {
//         break;
//       }
//
//       t -= vec[i];
//       n += 1;
//     }
//
//     r = r.max(n);
//   }
//
//   r as i32
// }


// 6

// 10
// 10
// 10

// 01

// 10
// 10

fn main() {
  let (mut n, mut k) = input_tuple_i32();
  let vec = input_vec_i32();

  let sol = solution(k, vec);

  send(sol);
}

fn send<T>(data: T)
where
    T: std::fmt::Display,
{
  println!("{}", data);
}


fn input() -> String {
  let mut input = String::new();

  io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

  return input.trim().to_string();
}

fn input_i32() -> i32 {
  let input = input();
  let mut n: i32 = input.trim().parse().unwrap();

  n
}

fn input_tuple_i32() -> (i32, i32) {
  let input = input();

  let (n, k): (i32, i32) = {
    let (a, b) = input.trim().split_once(" ").unwrap();

    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
  };

  return (n, k);
}

fn input_vec_i32() -> Vec<i32> {
  let input = input();

  let nums: Vec<i32> = input
      .split_whitespace()
      .map(|s| s.parse().expect("Not an integer!"))
      .collect();

  nums
}