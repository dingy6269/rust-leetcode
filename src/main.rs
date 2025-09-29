struct Solution {}

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let mut outcome: i32 = 0;

    let mut l = 0;
    let mut r = height.len() - 1;

    let mut l_max = height[l];
    let mut r_max = height[r];

    while l < r {
      let mut left = height[l];
      let mut right = height[r];

      if (l_max <= r_max) {
        l += 1;
        left = height[l];

        l_max = l_max.max(left);

        if left < l_max {
          let delta = l_max - left;

          outcome += delta;
        }
      } else {
        r -= 1;
        right = height[r];

        r_max = r_max.max(right);

        if right < r_max {
          let delta = r_max - right;

          outcome += delta;
        }
      }
    }

    outcome
  }
}

fn main() {
  let height: Vec<i32> = vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6];

  println!("{}", Solution::trap(height));
}
