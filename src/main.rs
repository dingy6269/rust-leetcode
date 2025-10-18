use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

//

struct Solution;

impl Solution {
  pub fn eval_rpn(tokens: Vec<&str>) -> i32 {
    let mut sum = 0;
    let mut stack: Vec<String> = vec![];

    let ops: HashMap<&str, fn(i32, i32) -> i32> =
      HashMap::from([
        ("+", (|x, y| x + y) as fn(i32, i32) -> i32),
        ("-", (|x, y| x - y) as fn(i32, i32) -> i32),
        ("*", (|x, y| x * y) as fn(i32, i32) -> i32),
        ("/", (|x, y| x / y) as fn(i32, i32) -> i32),
      ]);

    for token in tokens {
      if let Ok(n) = token.parse::<i32>() {
        stack.push(n.to_string());
      } else {
        println!("{:?}", stack);
        let n1 = stack.pop().map(|r| r.parse()).unwrap();
        let n2 = stack.pop().map(|r| r.parse()).unwrap();

        if let Some(cb) = ops.get(token) {
          let tmp = cb(n2.unwrap(), n1.unwrap());
          stack.push(tmp.to_string());
        }
      }
    }

    stack.last().unwrap().parse::<i32>().expect("int")
  }
}

fn main() {
  let tokens = vec!["4", "13", "5", "/", "+"];

  let ans = Solution::eval_rpn(tokens);

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

// fn solution(y: i32, w: i32) -> i32 {
//   ((6 - y.max(w)) + 1) as i32 / 6.0
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
