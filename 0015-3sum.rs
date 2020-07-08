struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();

        let n = nums.len();
        let mut i: usize = 0;
        while i < n {
            let target = -nums[i];
            let mut front = i + 1;
            let mut back = n - 1;
            while front < back {
                let sum = nums[front] + nums[back];
                if sum < target {
                    front += 1;
                } else if sum > target {
                    back -= 1;
                } else {
                    let triplet = vec![nums[i], nums[front], nums[back]];
                    // ignore duplicates of second number
                    while front < back && nums[front] == triplet[1] {
                        front += 1;
                    }
                    // ignore duplicates of third number
                    while front < back && nums[back] == triplet[2] {
                        back -= 1;
                    }
                    result.push(triplet);
                }
            }
            // ignore duplicates of first number
            while i + 1 < n && nums[i + 1] == nums[i] {
                i += 1;
            }

            i += 1;
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
    );
}
