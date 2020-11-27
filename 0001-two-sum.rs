use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match m.get(&(target - num)) {
            Some(&j) => return vec![j, i as i32],
            None => m.insert(num, i as i32),
        };
    }
    return vec![];
}

#[cfg(test)]
mod two_sum {
    use super::two_sum;

    #[test]
    pub fn finds_index_if_exists() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 18), vec![1, 2]);
    }
    #[test]
    pub fn no_solution() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 19), vec![]);
    }
}
