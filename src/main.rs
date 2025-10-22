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

fn solution(inp: String) -> String {
  let mut stack: Vec<String> = Vec::new();
  let s : Vec<i32> = inp.split('+').map(|s| s.parse::<i32>().unwrap()).collect();

  if (s.len() < 2 ) {
    return inp;
  }

  let pivot = &s[0];
  // let less = &s[..pivot];
  // let greater = &s[pivot..];

  let mut less = vec![];
  let mut greater = vec![];

  for x in &s[1..] {
    if x < pivot {
      less.push(x);
    } else {
      greater.push(x);
    }
  }

  let new = Vec::with_capacity(s.len());
  new.push(solution(less));
  new.push(x);
  new.push(solution(greater));

  new
}

fn main() {
  let inp = "2+1+2+2+2+3+1+3+1+2".to_string();


  let sol = solution(inp);

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