struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(c) = map.get(&(sum - k)) {
                count += c;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

fn main() {
    assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
}
