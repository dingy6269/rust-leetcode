use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set = nums.iter().cloned().collect::<HashSet<i32>>();
    let mut result: i32 = 0;
    
    for &num in &nums {
      if !set.contains(&(num - 1)) {
        let mut candidate = num + 1;

        loop {
          if !set.contains(&candidate) {
            break;
          }

          candidate += 1;
        }

        let delta = candidate - num;

        result = result.max(delta);
      }
    }

    result
  }
}

fn main() {
  let nums = vec![100, 4, 200, 1, 3, 2];
  
  let result = Solution::longest_consecutive(nums);

  println!("{:?}", result);
}
