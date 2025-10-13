use std::collections::HashSet;

struct Solution {}


// PRODUCT OF ARRAY EXCEPT SELF

impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut sum: Vec<i32> = vec![1; nums.len()];

    // 6 * 4 == 24
    // 2 * 3 * 4

    for (i, _) in nums.iter().enumerate() {
      for (j, v) in nums.iter().enumerate() {
        if i == j {
          continue 
        }

        sum[i] *= v;
      }
    }

   sum
  }
}


fn main() {
  let nums: Vec<i32> = vec![1, 2, 3, 4];

  let sol = Solution::product_except_self(nums);

  println!("{:#?}", sol);
}

