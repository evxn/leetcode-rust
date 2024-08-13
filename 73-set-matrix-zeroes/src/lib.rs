// https://leetcode.com/problems/set-matrix-zeroes/

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop, clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let first_row_has_zero = matrix[0].contains(&0);

        // mark the first elem at rows[1..] and cols[0..] containing zeros
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        // zero marked rows[1..]
        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }

        // zero marked cols[1..]
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }

        // zero the first col
        // note:  for some reason it's faster to do it here separately
        // instead of in the main rows zeroing loop
        // take this with a grain of salt since I haven't checked the generated asm
        // just benchmarked it on the leetcode
        if matrix[0][0] == 0 {
            for i in 1..m {
                matrix[i][0] = 0;
            }
        }

        // zero the first row
        if first_row_has_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut matrix: Vec<Vec<i32>> = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .into_iter()
            .map(Vec::from)
            .collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
                .into_iter()
                .map(Vec::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn example2() {
        let mut matrix: Vec<Vec<i32>> = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]]
            .into_iter()
            .map(Vec::from)
            .collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]
                .into_iter()
                .map(Vec::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn example3() {
        let mut matrix: Vec<Vec<i32>> = [[1, 0]].into_iter().map(Vec::from).collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[0, 0]].into_iter().map(Vec::from).collect::<Vec<_>>()
        );
    }

    #[test]
    fn example4() {
        let mut matrix: Vec<Vec<i32>> =
            [[1, 2, 3, 4], [5, 0, 7, 8], [0, 10, 11, 12], [13, 14, 15, 0]]
                .into_iter()
                .map(Vec::from)
                .collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[0, 0, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
                .into_iter()
                .map(Vec::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn example5() {
        let mut matrix: Vec<Vec<i32>> = [[1, 0, 3]].into_iter().map(Vec::from).collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[0, 0, 0]].into_iter().map(Vec::from).collect::<Vec<_>>()
        );
    }

    #[test]
    fn example6() {
        let mut matrix: Vec<Vec<i32>> = [
            [8, 3, 6, 9, 7, 8, 0, 6],
            [0, 3, 7, 0, 0, 4, 3, 8],
            [5, 3, 6, 7, 1, 6, 2, 6],
            [8, 7, 2, 5, 0, 6, 4, 0],
            [0, 2, 9, 9, 3, 9, 7, 3],
        ]
        .into_iter()
        .map(Vec::from)
        .collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 3, 6, 0, 0, 6, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0]
            ]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn example7() {
        let mut matrix: Vec<Vec<i32>> = [
            [-4, -2147483648, 6, -7, 0],
            [-8, 6, -8, -6, 0],
            [2147483647, 2, -9, -6, -10],
        ]
        .into_iter()
        .map(Vec::from)
        .collect();

        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [2147483647, 2, -9, -6, 0]]
                .into_iter()
                .map(Vec::from)
                .collect::<Vec<_>>()
        );
    }
}
