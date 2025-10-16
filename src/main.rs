// add - 0 traffic
// delete - 0 traffic
// send (al all and the sender) - (Lb to each where l is length)

// amount of traffic?

// 5
// 4 (including kate)


enum Command {
  Positive,
  Negative,
  Message { sender: String, name: String }
}

fn solution(commands: Vec<&str>) {
  for cmd in commands {
    match cmd.chars().next() {
      Some('+') => Command::Positive,
      Some('-') => Command::Negative,
      _ => {
        if let Some((sender, message)) = cmd.split_once(":") {
          Command::Message {
            sender: sender.to_string(), 
            name: message.to_string()
          }
        }
        else {
          panic!("Invalid format: {}" , cmd);
        }
      }
  };

  }
}


fn main() {
  let commands: Vec<&str> = vec![
        "+Mike", "Mike:hello", "+Kate", "+Dmitry",
        "-Dmitry", "Kate:hi", "-Kate"
    ];


 solution(commands);
}
