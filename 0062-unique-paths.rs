struct Solution;

impl Solution {
    // same as: ((m-1)+(n-1))! / ((m-1)! * (n-1)!)
    // or, (right + down)! / right! * down!
    //
    // nCk  = n! / ((n-k)! * k!)
    //      = [n * (n-1) *....* 1]  / [ ( (n-k) * (n-k-1) * .... * 1) *  ( k * (k-1) * .... * 1 ) ]
    //      = [n * (n-1) * .... * (n-k+1)] / [k * (k-1) * .... * 1]
    #[allow(non_snake_case)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as u64;
        let n = n as u64;

        let N = (n - 1) + (m - 1); // total steps
        let k = std::cmp::min(m - 1, n - 1);
        let mut result = 1 as u64;
        for i in 1..=k {
            result = result * (N - k + i) / i;
        }
        result as i32
    }
}

fn main() {
    assert_eq!(3, Solution::unique_paths(3, 2));
    assert_eq!(28, Solution::unique_paths(7, 3));
}
