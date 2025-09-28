use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        for row in board {
            let mut seen = HashSet::with_capacity(row.len());

            for cell in row {
                if seen.contains(&cell) {
                    return false;
                };

                seen.insert(cell);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn r1() {}
}
