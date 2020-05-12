struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut low = 0_usize;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if mid % 2 == 0 {
                if nums[mid] == nums[mid + 1] {
                    low = mid + 2;
                } else {
                    high = mid;
                }
            } else {
                if nums[mid] == nums[mid - 1] {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
        }
        nums[low]
    }
}

fn main() {
    assert_eq!(
        2,
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
    );
    assert_eq!(
        10,
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
    );
}
