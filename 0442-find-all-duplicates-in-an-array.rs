struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicates = Vec::new();
        for i in 0..nums.len() {
            let target = nums[i].abs() as usize - 1;
            if nums[target] < 0 {
                duplicates.push(target as i32 + 1);
            }
            nums[target] *= -1;
        }
        duplicates
    }
}

fn main() {
    assert_eq!(
        vec![2, 3],
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    assert_eq!(
        vec![10, 1],
        Solution::find_duplicates(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7])
    );
}
