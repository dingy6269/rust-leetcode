use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        let seen_width = board.get(0).map_or(0, |row| row.len());
        let mut seen_y : Vec<Vec<char>> = vec![Vec::new(), 9];

        for (ri, row) in board.into_iter().enumerate() {
            let mut seen_x = HashSet::with_capacity(row.len());

            for (ci, cell) in row.into_iter().enumerate() {
                if seen_x.contains(&cell) {
                    return false;
                };

                seen_y[ci].push(cell);

                seen_x.insert(cell);
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
