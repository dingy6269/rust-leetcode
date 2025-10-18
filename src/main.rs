use std::collections::HashSet;
use std::io::{self, BufRead};

const NODES_LEN: usize = 8;

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

fn solution(mut nodes: Vec<String>) -> i32 {
  let mut y_vec = vec![None; NODES_LEN];
  let mut x_vec = vec![None; NODES_LEN];

  let mut x_count = NODES_LEN;
  let mut y_count = NODES_LEN;

  // if y_vec i is None
  //

  // nodes
  for (i, node) in nodes.iter().enumerate() {
    let chars: Vec<char> =
      node.chars().take(NODES_LEN).collect();

    // W - [1]
    // x <= [1]
    // y <= [4]

    // chars
    for j in 0..NODES_LEN {
      let unit = chars[j];

      // - matters here
      if y_vec[i] != Some('-') {
        if y_vec[i].is_none() {
          y_vec[i] = Some(unit);
        } else {
          if unit != y_vec[i].unwrap() {
            y_count = y_count.saturating_sub(1);
            y_vec[i] = Some('-');
          }
        }
      }

      if x_vec[j] != Some('-') {
        if x_vec[j].is_none() {
          x_vec[j] = Some(unit);
        } else {
          if chars[j] != x_vec[j].unwrap() {
            x_count = x_count.saturating_sub(1);
            x_vec[j] = Some('-');
            // ??
          }
        }
      }
    }
  }

  //   dbg!(x_count);
  //   dbg!(y_count);

  //   dbg!(x_vec);
  //   dbg!(y_vec);

  (x_count + y_count) as i32
}

fn main() {
  let mut nodes = vec![];

  for _ in 0..8 {
    let i = input();
    nodes.push(i);
  }

  let sol = solution(nodes);

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

  input.trim().to_string()
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
