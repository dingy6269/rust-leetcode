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

// M * N cells
// 2 * 1 cell
// can be TURNED

//  1. D lie on the two cells
//  3. do not intersect
//  4. each domino lie inddie the board (with collisions)

// fn quicksort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
//   if list.len() < 2 {
//     list.to_vec()
//   } else {
//     let pivot = &list[0];
//
//     let mut less = vec![];
//     let mut greater = vec![];
//
//     for x in &list[1..] {
//       if x <= pivot {
//         less.push(x.clone());
//       } else {
//         greater.push(x.clone());
//       }
//     }
//
//     let mut new = Vec::with_capacity(list.len());
//     new.append(&mut quicksort(&less));
//     new.push(pivot.clone());
//     new.append(&mut quicksort(&greater));
//     new
//   }
// }

//


// 10
// (6 4)


// 5
// (3, 1, 2, 1)

// 5
// => (1, 1, 2, 3)

// 10
// 2 3 4 2 1 1


// 10
// 1 1 2 2 3 4

// (9)

fn solution(mut t: i32, mut vec: Vec<i32>) -> i32 {
  // if vec.len() == 1 {
  //   return if vec[0] <= t { 1 } else { 0 };
  // }

  vec.sort_by(|a, b| a.cmp(b));

  // dbg!(&vec);
  let mut n = 0;

  while (t > 0 && n < vec.len())  {
    dbg!(&vec);
    dbg!(t);
    dbg!(n);

    if t < vec[n] {
      break;
    }

    t -= vec[n];
    n += 1;
  };

  (n) as i32
}

// 6

// 10
// 10
// 10

// 01

// 10
// 10

fn main() {
  let (mut n, mut t) = input_tuple_i32();
  let vec = input_vec_i32();

  let sol = solution(t, vec);

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