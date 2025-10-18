use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

// daily temperatures
// answer[i]
// 

struct Solution;


impl Solution {
    pub fn daily_temperatures(temps: Vec<i32>) -> () {
        let mut result = vec![];

        for (i, val) in temps.iter().enumerate() {
            let mut count = 0;

            for j in i..(temps.len() -1) {
                if temps[j] <= *val {
                    count += 1;
                } else {
                    break;
                }
            };


            result.push(count);
        }
        // temps

        println!("{:?}", result);
    }
}

fn main() {
    Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]);
}
