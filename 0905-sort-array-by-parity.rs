struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut l = 0_usize;
        let mut r = a.len() - 1;
        while l < r {
            if a[l] % 2 == 0 {
                l += 1;
            } else {
                a.swap(l, r);
                r -= 1;
            }
        }
        a
    }
}

fn main() {
    assert_eq!(
        vec![4, 2, 1, 3],
        Solution::sort_array_by_parity(vec![3, 1, 2, 4])
    );
    assert_eq!(
        vec![1, 1, 1, 1],
        Solution::sort_array_by_parity(vec![1, 1, 1, 1])
    );
    assert_eq!(
        vec![2, 2, 2, 2],
        Solution::sort_array_by_parity(vec![2, 2, 2, 2])
    );
}
