use std::collections::HashSet;

struct Solution {}


// PRODUCT OF ARRAY EXCEPT SELF

impl Solution {
  pub fn contains_dublicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for num in nums {
      if set.contains(&num) {
        return true
      };
      set.insert(num);
    };

    false
  }

}

fn main() {
  let
}
