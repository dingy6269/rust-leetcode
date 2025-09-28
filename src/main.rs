use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - v)) {
                return vec![j, i as i32];
            }

            map.insert(v, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    fn t1() {
        // let mut book_reviews = HashMap::new();

        // book_reviews.insert
    }
}
