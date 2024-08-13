// https://leetcode.com/problems/trapping-rain-water

pub struct Solution;

// Time: O(n); Memory: O(1)
impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }

        struct MinOfWallsIterator {
            left_idx: usize,
            right_idx: usize,
            left_max_so_far: i32,
            right_max_so_far: i32,
            heights: Vec<i32>,
        }

        impl MinOfWallsIterator {
            pub fn from_heights(heights: Vec<i32>) -> Self {
                MinOfWallsIterator {
                    left_idx: 0,
                    left_max_so_far: heights[0],
                    right_idx: heights.len() - 1,
                    right_max_so_far: heights[heights.len() - 1],
                    heights,
                }
            }
        }

        struct MinOfWalls(i32);

        impl Iterator for MinOfWallsIterator {
            type Item = (i32, MinOfWalls);

            // we set who pointers and shifting the pointer corresponding to the lower max seen value.
            fn next(&mut self) -> Option<Self::Item> {
                if self.left_idx < self.right_idx {
                    let next_item = if self.left_max_so_far < self.right_max_so_far {
                        self.left_idx += 1;

                        let height = self.heights[self.left_idx];
                        self.left_max_so_far = self.left_max_so_far.max(height);

                        (height, MinOfWalls(self.left_max_so_far))
                    } else {
                        self.right_idx -= 1;

                        let height = self.heights[self.right_idx];
                        self.right_max_so_far = self.right_max_so_far.max(height);

                        (height, MinOfWalls(self.right_max_so_far))
                    };

                    return Some(next_item);
                }

                None
            }
        }

        let mut trapped = 0;

        for (height, MinOfWalls(min_of_walls)) in MinOfWallsIterator::from_heights(heights) {
            trapped += min_of_walls - height;
        }

        trapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example2() {
        let result = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(result, 9);
    }

    #[test]
    fn example3() {
        let result = Solution::trap(vec![]);
        assert_eq!(result, 0);
    }
}
