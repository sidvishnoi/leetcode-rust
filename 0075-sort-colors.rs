struct Solution;

// [2a,0a,2b,1a,1b,0b]
// [0b,0a,2b,1a,1b|2a]
// [0b|0a,2b,1a,1b|2a]
// [0b,0a|2b,1a,1b|2a]
// [0b,0a|1b,1a|2b,2a]
// [0b,0a,1b|1a|2b,2a]
// [0b,0a,1b,1a,2b,2a]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut zero = 0 as usize;
        let mut two = nums.len() - 1;
        let mut i = 0 as usize;
        while i <= two {
            match nums[i] {
                0 => {
                    nums.swap(i, zero);
                    i += 1;
                    zero += 1;
                }
                1 => {
                    i += 1;
                }
                2 => {
                    nums.swap(i, two);
                    if two == 0 {
                        return;
                    }
                    two -= 1;
                }
                _ => unreachable!(),
            };
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
}
