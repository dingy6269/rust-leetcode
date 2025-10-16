// h
// does not >= then h

// w 1, [mod] 2

// вв ряд

// min w

fn solution(h: i32, a: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in &a {
        if *num > h {
            count += 1;
        }
    }

    count + a.len();
};