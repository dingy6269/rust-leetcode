struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i1, v1) in nums.iter().enumerate() {
            for (i2, v2) in nums.iter().enumerate().skip(i1 + 1) {
                let sum = v1 + v2;

                if sum == target {
                    return vec![i1 as i32, i2 as i32];
                }
            }
        }

        vec![]
    }
}



fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator() {
        let v = vec!["a".to_string(), "b".to_string()];
        let mut v_iter = v.into_iter();

        let first_element: Option<String> = v_iter.next();

        assert_eq!(first_element, Some("a".to_string()));
        assert_eq!(v_iter.next(), Some("b".to_string()));
        assert_eq!(v_iter.next(), None);
    }
}
