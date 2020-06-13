struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }
        let mut nums = nums;

        // make sure each number has its divisors before it
        nums.sort();

        // dp[i] is size of divisible subset ending at nums[i]
        let mut dp = vec![1; nums.len()];
        let mut max_size = std::i32::MIN;
        for i in 1..nums.len() {
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 {
                    dp[i] = std::cmp::max(dp[i], 1 + dp[j]);
                }
            }
            max_size = std::cmp::max(max_size, dp[i]);
        }

        let mut result = Vec::new();
        for i in (0..nums.len()).rev() {
            if dp[i] == max_size {
                result.push(nums[i]);
                max_size -= 1;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![3, 1],
        Solution::largest_divisible_subset(vec![1, 2, 3])
    );
    assert_eq!(
        vec![8, 4, 2, 1],
        Solution::largest_divisible_subset(vec![1, 2, 4, 8])
    );
}
