struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0_usize;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if nums[mid] > nums[high] {
                low = mid + 1;
            } else if nums[mid] < nums[high] {
                high = mid;
            } else {
                high -= 1;
            }
        }
        nums[low]
    }
}

fn main() {
    assert_eq!(1, Solution::find_min(vec![1, 3, 5]));
    assert_eq!(0, Solution::find_min(vec![2, 2, 2, 0, 1]));
}
