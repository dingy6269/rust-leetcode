use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let width = board.get(0).map_or(0, |row| row.len());
    let width_3 = width / 3;

    dbg!(width);

    let mut seen_y: Vec<HashSet<char>> =
      vec![HashSet::with_capacity(width); width];

    let mut seen_3: Vec<Vec<HashSet<char>>> = vec![
        vec![HashSet::with_capacity(width); width_3];
        width_3
      ];

    for (ri, row) in board.into_iter().enumerate() {
      let mut seen_x = HashSet::with_capacity(row.len());

      for (ci, cell) in row.into_iter().enumerate() {
        let x_3 = ci / 3;
        let y_3 = ri / 3;

        if !cell.is_numeric() {
          continue;
        }

        if seen_x.contains(&cell) {
          return false;
        };

        if seen_y[ci].contains(&cell) {
          return false;
        }

        if seen_3[x_3][y_3].contains(&cell) {
          return false;
        }

        seen_x.insert(cell);
        seen_y[ci].insert(cell);
        seen_3[x_3][y_3].insert(cell);
      }
    }

    true
  }
}

// trait IsNumeric {
//     fn is_numeric(&self) -> bool;
// }

// impl IsNumeric for &str {
//     fn is_numeric(&self) -> bool {
//         self.parse::<i32>().is_ok()
//     }
// }

// impl IsNumeric for char {
//     fn is_numeric(&self) -> bool {
//         self.is_ascii_digit()
//     }
// }

fn main() {
  let x = 2 / 3; // 0
  let y = 3 / 3; // 1 
  // should be 0

  // let r = [[[], []], [], []];

  // let stuff = HashMap::[
  //     ()
  // ]

  let group = (3) / 3;

  println!("{}", group);

  let board = vec![
    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
  ];

  let result = Solution::is_valid_sudoku(board);

  println!("result {}", result);
  assert!(result);
}

#[cfg(test)]
mod tests {

  #[test]
  fn r1() {}
}
