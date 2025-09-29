struct Solution {}

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let mut outcome: i32 = 0;

    let mut l = 0;
    let mut r = height.len() - 1;

    let mut l_max = 0;
    let mut r_max = 0;

    while l <= r {
      let left = height[l];
      let right = height[r];

      l_max = l_max.max(left);
      r_max = r_max.max(right);

      let level = r_max.min(l_max);

      if right < r_max {
        let delta = level - right;

        outcome += delta;
      }

      if left < l_max {
        let delta = level - left;

        outcome += delta;
      }

      l += 1;
      r -= 1;
    }

    outcome
  }
}

fn main() {
  let height: Vec<i32> =
    vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

  println!("{}", Solution::trap(height));
}
