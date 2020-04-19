struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        Self::search_rotated(&nums, target, 0_usize, nums.len() - 1)
    }

    fn search_rotated(nums: &Vec<i32>, target: i32, low: usize, high: usize) -> i32 {
        if low > high {
            return -1;
        }

        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[low] <= nums[mid] {
            // if left half is sorted
            if nums[low] <= target && nums[mid] >= target {
                // if target in left half of left half
                Self::search_rotated(nums, target, low, mid - 1)
            } else {
                // search in right of left half
                Self::search_rotated(nums, target, mid + 1, high)
            }
        } else {
            // if right half is sorted
            if nums[mid] <= target && nums[high] >= target {
                // if target in right half
                Self::search_rotated(nums, target, mid + 1, high)
            } else {
                // search left of right half
                Self::search_rotated(nums, target, low, mid - 1)
            }
        }
    }
}

fn main() {
    assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
}
