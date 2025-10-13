// car fleet;

// n cars (from mile 0) travellign to target

// pos[]
// speed[]

// cannot catch up

// speed of car fleet = minimum speed of any car

// (if a car to traget it still a fleet)

// target 12
// position 10, 8, 0, 5, 3
// speed 2, 4, 1, 1, 3

// 3

// 10 + 2
// 8  + 4
// ==>> 12

// 5 + 1
// 3 + 3
// => 6 (1)

// -----
// target 100
// 0, 2, 4
// 4, 2, 1
// =>
// 4, 4, 5
// 8, 6, 6

// Дединский курс

struct Solution {}

impl Solution {
  pub fn car_fleet(
    target: i32,
    position: Vec<i32>,
    speed: Vec<i32>,
  ) -> i32 {
    let mut cars = position
      .iter()
      .enumerate()
      .map(|(idx, pos)| (pos, &speed[idx]))
      .collect::<Vec<(&i32, &i32)>>();

    // we need to sort it
    // yeah but why?

    //

    cars.sort_by(|a, b| a.cmp(b));

    let mut stack: Vec<i32> = Vec::new();

    for &(pos, speed) in cars.iter().rev() {
      println!("pos={:?}, speed={:?}", pos, speed);
      let time = (target - pos) as f64 / (*speed as f64);

      // 4 / 3 (rust tells 1) // well it is 1.1111
      println!("time={:?}", time);
      println!("stack={:?}", stack);

      if let Some(&last) = stack.last() {
        if time <= last as f64 {
          continue;
        }
      }

      // ???
      stack.push(time as i32);
      // stack.push(time);

      println!("--------");
    }

    return stack.len() as i32;
  }
}

fn main() {
  // Solution::car_fleet(
  //   12,
  //   vec![10, 8, 0, 5, 3, 4],
  //   vec![2, 4, 1, 1, 3, 1],
  // );

  // Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]);

  let sol =  Solution::car_fleet(10, vec![6, 8], vec![3, 2]);

  println!("{:?}", sol);
}
