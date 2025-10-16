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


// next round if
// >= points then another guy who k place
// IF (!!) his points are > 0

// n guys
// (with points)

// 8 5
// 10 9 8 7 7 7 5 5



fn solution(nums: Vec<i32>, k: usize) -> i32 {
  let mut count = 0;

  k.saturating_sub

  for num in &nums {
    if *num >= nums[k - 1] && *num > 0 {
      count += 1;
    }
  };

  count
}

fn main() {
  let (n, k) = input_tuple_i32();

  let points = input_vec_i32();

  println!("{}", solution(points, k as usize));
}


fn input() -> String {
  let mut input = String::new();
 
  io::stdin().read_line(
        &mut input
  ).expect("Failed to read line");

  return input
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