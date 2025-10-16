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


// p v t
// n tasks

// at least (2) got it and sure with solution
// or not write

fn solution(nums: Vec<i32>) -> i32 {
  if nums.iter().filter(|&&x| x == 1).count() >= 2 {
    1
  } else {
    0
  }
  
}

fn main() {
  let mut input = String::new();
  let mut result: i32 = 0;

  io::stdin().read_line(&mut input).unwrap();

  let mut n: i32 = input.trim().parse().unwrap();

  while (n > 0) {
    let mut input = String::new();
 
    io::stdin().read_line(
        &mut input
    ).expect("Failed to read line");
 
 
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Not an integer!"))
        .collect();

    result += solution(nums);

    n -= 1;
  }

  println!("{}", result);
}
