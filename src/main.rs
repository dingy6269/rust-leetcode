// add - 0 traffic
// delete - 0 traffic
// send (al all and the sender) - (Lb to each where l is length)

// amount of traffic?

// 5
// 4 (including kate)

use std::collections::HashSet;

fn solution(commands: Vec<&str>) -> i32 {
  let mut people: HashSet<&str> = HashSet::new();
  let mut result: i32 = 0;

  for cmd in commands {
    match cmd.chars().next() {
      Some('+') => {
        people.insert(&cmd[1..]);
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
  let commands = vec![
    "+Mike",
    "-Mike",
    "+Mike",
    "Mike:Hi   I am here",
    "-Mike",
    "+Kate",
    "-Kate",
  ];

  let sol = solution(commands);

  println!("{}", sol);
}
