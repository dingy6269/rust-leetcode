struct Solution {}

impl Solution {
  pub fn two_sum(
    numbers: Vec<i32>,
    target: i32,
  ) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left <= right {
      let candidate = numbers[left] + numbers[right];

      if candidate == target {
        let (l, r): (i32, i32) =
          ((left + 1) as i32, (right + 1) as i32);

        return vec![l, r];
      };

      if candidate > target {
        right -= 1
      }

      if candidate < target {
        left += 1
      }
    }

    panic!("shouldn't happen really");
  }
}

fn main() {
  let numbers = vec![2, 7, 11, 15];
  let target = 9;

  let result = Solution::two_sum(numbers, target);

  println!("{:?}", result);
}
