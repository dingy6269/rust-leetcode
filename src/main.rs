struct Solution {}


// square: n x m
// plot: a x a


impl Solution {
  pub fn square(
    n: i32, m: i32, a: i32
  ) -> i32 {
    // 36
    let square = n * m;
    // 16
    let plot = a * a;
    // 3
    

    // -----
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
  let result = Solution::square();

  println!("{:?}", result);
}
