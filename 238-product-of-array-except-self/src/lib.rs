// https://leetcode.com/problems/product-of-array-except-self

pub struct Solution;

impl Solution {
    // The problem can be solved sequentially in a simpler manner,
    // but this solution uses iterators and parallelizm,
    // which scales better with `nums` length growth
    // and more resembles a real production codebase.
    // Note that a single function is used to compute both prefix
    // and postfix partial products. And a direction of a single pass
    // is expressed with standard `fold` and `rfold` iterator methods.
    // Time: O(n); Memory: O(n)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums = std::sync::Arc::new(nums);

        let jh = std::thread::spawn({
            let nums = nums.clone();
            move || {
                let (products_after_elem, _) = nums
                    .iter()
                    .enumerate()
                    // go backwards and compute products of elems going after the current
                    .rfold((vec![0; nums.len()], 1), Solution::compute_partial_products);

                products_after_elem
            }
        });

        let (products_before_elem, _) = nums
            .iter()
            .enumerate()
            // compute products of elems going before the current
            .fold((vec![0; nums.len()], 1), Solution::compute_partial_products);

        let products_after_elem = jh.join().expect("pre-computed partial products");

        products_before_elem
            .into_iter()
            .zip(products_after_elem)
            .map(|(product_before, product_after)| product_before * product_after)
            .collect()
    }

    fn compute_partial_products(
        (mut products, mut product_acc): (Vec<i32>, i32),
        (index, &num): (usize, &i32),
    ) -> (Vec<i32>, i32) {
        products[index] = product_acc;
        product_acc *= num;

        (products, product_acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3, 4];
        let output = Solution::product_except_self(input);
        assert_eq!(&output, &[24, 12, 8, 6]);
    }

    #[test]
    fn example2() {
        let input = vec![-1, 0, 1, 2, 3];
        let output = Solution::product_except_self(input);
        assert_eq!(&output, &[0, -6, 0, 0, 0]);
    }

    #[test]
    fn example3() {
        let input = vec![-1, 1, 0, -3, 3];
        let output = Solution::product_except_self(input);
        assert_eq!(&output, &[0, 0, 9, 0, 0]);
    }
}
