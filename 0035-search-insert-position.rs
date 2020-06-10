struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = (nums.len() as i32) - 1;
        while low <= high {
            let mid = (low + (high - low) / 2) as usize;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > target {
                high = (mid as i32) - 1;
            } else {
                low = (mid as i32) + 1;
            }
        }
        low
    }
}

fn main() {
    assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
}
