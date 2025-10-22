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

// (x, y)

fn solution(matrix: &Vec<Vec<i32>>, init: (i32, i32)) -> i32 {
  let cond = true;
  let coords = init.clone();
  const target: (i32, i32) = (2, 2);

  while cond {
    let delta_x = coords.0 - target.0;
    let delta_y = coords.1 - target.1;

    if (delta_x > 0) {
      
    }


  }

  0
}

fn main() {
  let mut matrix: Vec<Vec<i32>> = Vec::new();
  let mut init: Option<(i32, i32)> = None;

  for i in 0..5 {
    let x = input_vec_i32();

    for (j, num) in x.iter().enumerate() {
      if *num == 1 {
        init = Some((
          i, j as i32
        ));
      }
    }
    matrix.push(x);
  }

  let sol = solution(&matrix);

  send(sol);
}



fn send<T>(data: T) where T: std::fmt::Display {
  println!("{}", data);
}


fn input() -> String {
  let mut input = String::new();

  io::stdin().read_line(
        &mut input
  ).expect("Failed to read line");

  return input.trim().to_string()
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

  return (n, k)
}

fn input_vec_i32() -> Vec<i32> {
   let input = input();

    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Not an integer!"))
        .collect();

    nums
}