use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

// daily temperatures
// answer[i]
// 

struct Solution;


impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // 2
        let corr = h as f64 / (piles.len() as f64);

        let min = i32::MAX;
        let max = i32::MIN;

        for (i, pile) in piles.into_iter().enumerate() {
            let cel = pile as f64 / corr;

            min = min.min(cel.floor() as i32);
            max = max.max(cel.ceil() as i32);
        };
    }
}

fn main() {

}

// [3, 6, 7, 11]

// 3 

// 4
// 2

// 4
// 3

// 4
// 4
// 3


// corr h / len

// 8 / 4 = 2


// 1.5
// 3
// 3.5
// 5.5

// 1..6

// =====

// 5 /5 = 1

// 30
// 11
// 23
// 4 
// 20 

// =============

// corr h / len 

// 5 / 5 == 1

// 30 ?? 1 (mod)
// 30

// 11 ?? 1 (mod)

// corr h / len

// 6 / 5 == 1.25
// 