use std::collections::HashMap;
use std::process::ExitCode;

impl Solution {
  pub fn group_anagrams(strs: Vec<String>) {
    let result: HashMap<HashMap<char, i32>, Vec<String>> = HashMap::default();

    for str in strs {
      let outcome = str.as_str().count();

      *result
      .entry()
      .and_modify(|e| {
        
      })
    }
  }

  pub fn count(s: &str) {
    let mut counts = HashMap::new();

    for ch in s.chars() {
      *counts.entry(ch).or_insert_with(|| 0) += 1;
    }

    println!("{:?}", counts);
  }
}

pub trait CharCount {
  fn count(&self) -> HashMap<char, i32>;
}

impl CharCount for &str {
  fn count(&self) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for ch in self.chars() {
      *counts.entry(ch).or_insert_with(|| 0) += 1;
    }

    counts
  }
}

struct Solution {}

fn main() {
  let sol = Solution::count("ana");

  println!("{:?}", sol);
}
