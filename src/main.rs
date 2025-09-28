fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn r1() {
        let str = "MCMXCIV";
        let mut result = 0;

        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let subt_map = HashMap::from([
            ("IV", 4),
            ("IX", 9),
            ("XL", 40),
            ("XC", 90),
            ("CD", 400),
            ("CM", 900),
        ]);

        let mut stack = Vec::new();

        for char in str.chars() {
            let sb = stack.pop();

            if matches!(char, 'I' | 'X' | 'C') {
                stack.push(char);
            }

            if let Some(last) = sb {
                let key = [last, char].iter().collect::<String>();

                if let Some(sb_value) = subt_map.get(&key.as_str()) {
                    result -= roman_map.get(&last).unwrap();
                    result += sb_value;
                    continue;
                } 
            }

            if let Some(value) = roman_map.get(&char) {
                result += value
            }

        }

        println!("{:?}", result);

        assert_eq!(result, 1994);
    }

    #[test]
    fn r2() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

        let even_numbers = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect::<Vec<i32>>();

        println!("{:?}", even_numbers);
    }
}



use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn r1() {
        let str = "MCMXCIV";
        let mut result = 0;

        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let subt_map = HashMap::from([
            ("IV", 4),
            ("IX", 9),
            ("XL", 40),
            ("XC", 90),
            ("CD", 400),
            ("CM", 900),
        ]);

        let mut stack = Vec::new();

        for char in str.chars() {
            let sb = stack.pop();

            if matches!(char, 'I' | 'X' | 'C') {
                stack.push(char);
            }

            if let Some(last) = sb {
                let key = [last, char].iter().collect::<String>();

                if let Some(sb_value) = subt_map.get(&key.as_str()) {
                    result -= roman_map.get(&last).unwrap();
                    result += sb_value;
                    continue;
                } 
            }

            if let Some(value) = roman_map.get(&char) {
                result += value
            }

        }

        assert_eq!(result, 1994);
    }
}