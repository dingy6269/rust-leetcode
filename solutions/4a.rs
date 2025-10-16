// add - 0 traffic
// delete - 0 traffic
// send (al all and the sender) - (Lb to each where l is length)

// amount of traffic?

// 5
// 4 (including kate)

use std::collections::HashSet;
use std::io::{self, BufRead};

fn solution(commands: Vec<String>) -> i32 {
  let mut people: HashSet<String> = HashSet::new();
  let mut result: i32 = 0;

  for cmd in commands {
    match cmd.chars().next() {
      Some('+') => {
        people.insert(cmd[1..].to_string());
      }
      Some('-') => {
        people.remove(&cmd[1..]);
      }
      _ => {
        if let Some((sender, message)) = cmd.split_once(":")
        {
          let tmp = (message.len() * people.len());
          result += tmp as i32;
        } else {
          panic!("Invalid format: {}", cmd);
        }
      }
    };
  }
  result
}

fn main() {
  let mut input = String::new();
  let stdin = io::stdin();
  let mut commands: Vec<String> = vec![];

  for line in stdin.lock().lines() {
    let line = line.unwrap();

    commands.push(line);
  }

  let sol = solution(commands);

  println!("{}", sol);
}
