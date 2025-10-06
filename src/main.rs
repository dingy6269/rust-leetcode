use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // full value here
    let set = nums.iter().cloned().collect::<HashSet<i32>>();
    let mut result: i32 = 0;
    
    for (idx, val) in nums.iter().enumerate() {
      if idx == 0 || !set.contains(&(nums[idx] - 1)) {
        let mut sequence = 1;
        let mut candidate = nums[idx] + sequence;

        while set.contains(&candidate) {
          sequence += 1;
          candidate = nums[idx] + sequence;
        }


        result = result.max(sequence as i32);
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
