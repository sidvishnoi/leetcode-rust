struct Solution;

// Credits: https://leetcode.com/problems/counting-bits/discuss/270693/
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = num as usize;
        let mut result = Vec::with_capacity(num + 1);
        result.push(0);
        for i in 1..=num {
            if i % 2 == 0 {
                // if i is even, it's last bit is 0.
                // so, count_bits(num) = count_bits(num >> 1)
                // i.e., we ignore last bit
                result.push(result[i >> 1]);
            } else {
                // if i is odd, it's last bit is 1.
                // so, count_bits(num) = count_bits(num - 1) + 1
                // num:     101010101011
                // num - 1: 101010101010
                result.push(result[i - 1] + 1);
            }
        }
        result
    }
}

fn main() {
    assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
}
