struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        // i'th person trusts trusts[i] persons and is trusted by trusted_by[i] persons.
        // then we find the person `i` with trusts[i] == 0 && trusted_by[i] == n-1
        // we can optimize for space as we only care about count[i] = trusted_by[i] - trusts[i]
        let mut counts = vec![0; n + 1];
        for t in trust {
            counts[t[0] as usize] -= 1;
            counts[t[1] as usize] += 1;
        }

        match counts.into_iter().skip(1).position(|c| c as usize == n - 1) {
            Some(i) => (i + 1) as i32,
            None => -1
        }
    }
}

fn main() {
    let trust = vec![vec![1, 2]];
    assert_eq!(2, Solution::find_judge(2, trust));

    let trust = vec![vec![1, 3], vec![2, 3]];
    assert_eq!(3, Solution::find_judge(3, trust));

    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    assert_eq!(-1, Solution::find_judge(3, trust));

    let trust = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(-1, Solution::find_judge(3, trust));

    let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
    assert_eq!(3, Solution::find_judge(4, trust));
}
