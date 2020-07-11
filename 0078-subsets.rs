struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
        result.push(Vec::new());

        for num in nums {
            let size = result.len();
            for i in 0..size {
                let mut subset = result[i].clone();
                subset.push(num);
                result.push(subset);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ],
        Solution::subsets(vec![1, 2, 3])
    );
}
