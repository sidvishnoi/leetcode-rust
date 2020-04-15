struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        // result[i] contains the product of all the elements to the left.
        result.push(1); // there are no elements to the left, result[0] = 1
        for i in 1..nums.len() {
            result.push(nums[i - 1] * result[i - 1]);
        }

        // right_product is the product of all the elements to the right
        let mut right_product = 1;
        for i in (0..nums.len()).rev() {
            result[i] = result[i] * right_product;
            right_product *= nums[i];
        }

        result
    }
}

fn main() {
    assert_eq!(
        vec![24, 12, 8, 6],
        Solution::product_except_self(vec![1, 2, 3, 4])
    );
}
