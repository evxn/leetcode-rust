// https://leetcode.com/problems/valid-sudoku

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let mut sq_3x3_sets = vec![vec![HashSet::new(); 3]; 3];
        let mut row_sets = vec![HashSet::new(); 9];
        let mut col_sets = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    '.' => {
                        continue;
                    }
                    val => {
                        if sq_3x3_sets[i / 3][j / 3].contains(&val)
                            || row_sets[i].contains(&val)
                            || col_sets[j].contains(&val)
                        {
                            return false;
                        }

                        sq_3x3_sets[i / 3][j / 3].insert(val);
                        row_sets[i].insert(val);
                        col_sets[j].insert(val);
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
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
        assert_eq!(Solution::is_valid_sudoku(input), true);
    }

    #[test]
    fn example2() {
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(input), false);
    }
}
