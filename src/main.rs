use std::collections::HashSet;
use std::io::{self, BufRead};

// sorted matrix
// so binary search
// lets go

// it could be matrix any size
// this is just an example

// so yeah binary search
// absolutely
// this thiing is sorted +
// so in 2d matrixes this is it

struct Solution;

impl Solution {
  pub fn search_matrix(
    matrix: Vec<Vec<i32>>,
    target: i32,
  ) -> bool {
    let mut left = 0;
    let mut right = matrix.len() - 1;

    let mut arr = None;

    // O(log n)
    while left <= right {
      // 0 + 2  //// 2 => 1
      let mid = (left + right) / 2;

      let first = matrix[mid][0];
      // refactor
      let last = matrix[mid].last().unwrap().clone();

      if last < target {
        left = mid + 1;
      }

      if first > target {
        right = mid  - 1;
      }

      if target > first && target < last {
        arr = Some(mid);
        break;
      }
    };


    if (arr.is_none()) {
        return false;
    }

    
    let tarr = &matrix[arr.unwrap()];
    let mut left = 0;
    let mut right = tarr.len() - 1;

    while left <= right {
        let mut mid = (left + right) / 2;

        if (target > tarr[mid]) {
            left = mid + 1 
        }

        if (target < tarr[mid]) {
            right = mid - 1
        }

        if (target == tarr[mid]) {
            return true
        }
    }

    false
  }
}

fn main() {
  let matrix: Vec<Vec<i32>> = vec![
    vec![1, 3, 5, 7],
    vec![10, 11, 16, 20],
    vec![23, 30, 34, 60],
  ];

  let ans = Solution::search_matrix(matrix, 3);

  println!("{:?}", ans);
}

// 8x8
// vertical or horizontal

// const NODES_LEN: usize = 8;

// fn solution(nodes: Vec<String>) -> i32 {
//     let mut x_vec: Vec<Option<char>> = vec![None; NODES_LEN];
//     let mut y_vec: Vec<Option<char>> = vec![None; NODES_LEN];

//     for node in nodes.iter() {
//         let chars: Vec<char> = node.chars().collect();

//         for ch in 0..NODES_LEN {
//             if x_vec[ch].is_none() {
//                 x_vec[ch] = Some(chars[ch]);
//             }
//         }
//     }

//     0
// }

// fn solution(y: i32, w: i32) -> f64 {
//   ((6 - y.max(w)) + 1) as f64 / 6.0
// }

// // nothing stops you from now
// // reload your mind

// fn main() {
//   let (y, w) = input_tuple_i32();

//   let sol = solution(y, w);

//   send(sol);
// }

// fn send<T>(data: T)
// where
//   T: std::fmt::Display,
// {
//   println!("{}", data);
// }

// fn input() -> String {
//   let mut input = String::new();

//   io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read line");

//   input.trim().to_string()
// }

// fn input_i32() -> i32 {
//   let input = input();
//   let mut n: i32 = input.trim().parse().unwrap();

//   n
// }

// fn input_tuple_i32() -> (i32, i32) {
//   let input = input();

//   let (n, k): (i32, i32) = {
//     let (a, b) = input.trim().split_once(" ").unwrap();

//     (a.trim().parse().unwrap(), b.trim().parse().unwrap())
//   };

//   return (n, k);
// }

// fn input_vec_i32() -> Vec<i32> {
//   let input = input();

//   let nums: Vec<i32> = input
//     .split_whitespace()
//     .map(|s| s.parse().expect("Not an integer!"))
//     .collect();

//   nums
// }
