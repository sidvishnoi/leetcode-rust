struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();

        // [0,1,0,3,12] => [1,1,0,3,12] => [1,3,0,3,12] => [1,3,12,3,12]
        let mut last_non_zero_found_at: usize = 0;
        for i in 0..n {
            if nums[i] != 0 {
                nums[last_non_zero_found_at] = nums[i];
                last_non_zero_found_at += 1;
            }
        }

        // => [1,3,12,3,12] => [1,3,12,0,0]
        for i in last_non_zero_found_at..n {
            nums[i] = 0;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![1, 3, 12, 0, 0], nums);
}
