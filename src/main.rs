use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // full value here
    let set: HashSet<i32> = nums.iter().cloned().collect();
    let mut result: i32 = 0;

    for (idx, val) in nums.iter().enumerate() {
      let decrement = val - 1;

      if (!set.contains(&decrement)) {
        let mut sequence = 1;
        


        while nums
          .get(idx + sequence)
          .map_or(false, |v| set.contains(v))
        {
          sequence += 1
        }

        result = result.max(sequence as i32);
      }
    }

    result
  }
}

fn main() {
  let nums = vec![100, 4, 200, 1, 3, 2];
  println!("{:#?}", Solution::longest_consecutive(nums));
}
