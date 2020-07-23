struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Let the two numbers be a and b.
        // xor = a^b, as all other duplicate numbers cancel each other out.
        let xor = nums.iter().fold(0, |acc, num| acc ^ num);

        // Find the mask with lowest set bit of xor. a and b differ in this bit
        // atleast.
        let last_set_bit = xor & (-xor);

        let (mut a, mut b) = (0, 0);
        // Based on the mask, group the items into two groups. One contains a,
        // other contains b.
        for num in nums {
            if num & last_set_bit != 0 {
                a = a ^ num;
            } else {
                b = b ^ num;
            }
        }

        vec![a, b]
    }
}

fn main() {
    assert_eq!(vec![3, 5], Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
}
