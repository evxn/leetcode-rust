// https://leetcode.com/problems/number-of-islands/
pub struct Solution;

use std::cmp;
use std::collections::{HashSet, VecDeque};

enum Direction {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
}

impl Solution {
    // naturally iterate over the grid left to right, top to bottom
    // if curent cell contains 1 then start the search:
    // add right and down directions to the traversal_later queue
    // drain traversal_later queue
    // if current direction is right then keep moving right
    // also looking up and down til you hit 0 or a visited node
    // if current direction is left then keep moving left
    // also looking up and down til you hit 0 or a visited node
    // if current direction is down then keep moving down
    // also looking left and right til you hit 0 or visited
    // mark visited nodes
    // when queue is empty increment islands counter
    // and continue natural iteration over the grid skipping visited nodes
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut traverse_later: VecDeque<Direction> = VecDeque::new();
        let mut visited = HashSet::with_capacity(m * n);
        let mut islands_count = 0;

        for i in 0..m {
            for j in 0..n {
                if visited.contains(&(i, j)) {
                    continue;
                }

                visited.insert((i, j));

                if grid[i][j] == '0' {
                    continue;
                }

                // new island found - search it!

                traverse_later.push_back(Direction::Right(i, j));
                traverse_later.push_back(Direction::Down(i, j));

                while let Some(direction) = traverse_later.pop_front() {
                    // get clamped cell coordinates in the direction
                    let (i, j) = match direction {
                        Direction::Left(i, j) => (i, j.saturating_sub(1)),
                        Direction::Right(i, j) => (i, cmp::min(j + 1, n - 1)),
                        Direction::Up(i, j) => (i.saturating_sub(1), j),
                        Direction::Down(i, j) => (cmp::min(i + 1, m - 1), j),
                    };

                    if visited.contains(&(i, j)) {
                        continue;
                    }

                    visited.insert((i, j));

                    if grid[i][j] == '0' {
                        continue;
                    }

                    match direction {
                        Direction::Left(_, _) => {
                            traverse_later.push_back(Direction::Left(i, j));
                            traverse_later.push_back(Direction::Up(i, j));
                            traverse_later.push_back(Direction::Down(i, j));
                        }
                        Direction::Right(_, _) => {
                            traverse_later.push_back(Direction::Right(i, j));
                            traverse_later.push_back(Direction::Up(i, j));
                            traverse_later.push_back(Direction::Down(i, j));
                        }
                        Direction::Up(_, _) => {
                            traverse_later.push_back(Direction::Up(i, j));
                            traverse_later.push_back(Direction::Left(i, j));
                            traverse_later.push_back(Direction::Right(i, j));
                        }
                        Direction::Down(_, _) => {
                            traverse_later.push_back(Direction::Down(i, j));
                            traverse_later.push_back(Direction::Left(i, j));
                            traverse_later.push_back(Direction::Right(i, j));
                        }
                    }
                }

                // no more undiscovered cells on the island
                islands_count += 1;
            }
        }

        islands_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn example3() {
        let grid = vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
            vec!['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '1', '1', '1', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 2);
    }
}
