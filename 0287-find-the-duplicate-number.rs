struct Solution;

impl Solution {
    // Using Floyd's Tortoise and Hare (Cycle Detection) algorithm
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Find the intersection point of the two runners.
        let mut slow = nums[0];
        let mut fast = nums[0];
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }

        // Find cycle entrance
        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }
        slow
    }
}

fn main() {
    assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
}
