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

fn solution(line: String) -> String {
  if (line.len() > 10) {
    let b = line.trim().as_bytes();

    return format!("{}{}{}", b[0] as char, b.len() - 2, b[b.len() -1] as char);
  } else {
    return line;
  }
}

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut n: i32 = input.trim().parse().unwrap();


  while (n > 0) {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    println!("{}", solution(t));

    n -= 1;
  }
}
