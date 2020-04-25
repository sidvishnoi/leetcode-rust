struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut leftmost_good_position = nums.len() - 1;
        for pos in (0..nums.len()).rev() {
            if pos as i32 + nums[pos] >= leftmost_good_position as i32 {
                leftmost_good_position = pos;
            }
        }
        leftmost_good_position == 0
    }
}

fn main() {
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
