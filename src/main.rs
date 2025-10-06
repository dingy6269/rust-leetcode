struct Solution {}


// square: n x m
// plot: a x a


impl Solution {
  pub fn square(
    n: i32, m: i32, a: i32
  ) -> i32 {
    // 36
    // x >>>
    // y >>> 
    let square = n * m;
    // 16
    let plot = a * a;
    // 3

    let base = (
      square * 4
    ) as f64 / (plot * 4) as f64;

    // we would have 3
    let mut solution = base.ceil() as i32;

    while solution % 2 != 0 {
      solution += 1
    };
    
    solution
}
}

fn main() {
  let result = Solution::square(6, 6, 4);

  println!("{:?}", result);
}
