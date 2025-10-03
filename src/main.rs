use std::collections::HashMap;
use std::process::ExitCode;

impl Solution {
  pub fn group_anagrams(strs: Vec<String>) {
  }

  pub fn is_anagram(str: String) {
    let mut chars = HashMap::new();

    for char in str.chars() {
      if !chars.contains_key(&char) {
        chars.insert(char, 0);
      }

      let candidate =
        chars.get(&char).unwrap_or_else(|| &0);

      let increment = candidate + 1;
      chars.insert(char, increment);
    }

    println!("{:?}", chars);
  }
}

// trait IsAnagram for &str {

// }

struct Solution {}


fn main() {
  let sol = Solution::is_anagram("ana".to_string());

  println!("{:?}", sol);
}