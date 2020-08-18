struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        }
        let mut results = Vec::new();
        for num in 1..10 {
            Self::dfs(n - 1, num, k, &mut results);
        }
        results
    }

    fn dfs(n: i32, num: i32, k: i32, results: &mut Vec<i32>) {
        if n == 0 {
            results.push(num);
            return;
        }

        let mut next_digits = Vec::new();
        let tail_digit = num % 10;
        next_digits.push(tail_digit + k);
        if k != 0 {
            next_digits.push(tail_digit - k);
        }
        for next_digit in next_digits {
            if matches!(next_digit, 0..=9) {
                let new_num = num * 10 + next_digit;
                Self::dfs(n - 1, new_num, k, results);
            }
        }
    }
}

macro_rules! sorted {
    ($vector:expr) => {{
        let mut tmp = $vector;
        tmp.sort();
        tmp
    }};
}

fn main() {
    assert_eq!(
        sorted!(vec![181, 292, 707, 929, 818]),
        sorted!(Solution::nums_same_consec_diff(3, 7))
    );
    assert_eq!(
        sorted!(vec![
            135, 131, 246, 242, 202, 357, 353, 313, 468, 464, 424, 420, 579, 575, 535, 531, 686,
            646, 642, 797, 757, 753, 868, 864, 979, 975
        ]),
        sorted!(Solution::nums_same_consec_diff(3, 2))
    );
    assert_eq!(
        sorted!(vec![
            12, 10, 23, 21, 34, 32, 45, 43, 56, 54, 67, 65, 78, 76, 89, 87, 98
        ]),
        sorted!(Solution::nums_same_consec_diff(2, 1))
    );
    assert_eq!(
        sorted!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        sorted!(Solution::nums_same_consec_diff(1, 1))
    );
    assert_eq!(
        sorted!(vec![18, 29, 70, 81, 92]),
        sorted!(Solution::nums_same_consec_diff(2, 7))
    );
    assert_eq!(
        sorted!(vec![11, 22, 33, 44, 55, 66, 77, 88, 99]),
        sorted!(Solution::nums_same_consec_diff(2, 0))
    );
    assert_eq!(
        sorted!(vec![90]),
        sorted!(Solution::nums_same_consec_diff(2, 9))
    );
    assert_eq!(
        sorted!(vec![909]),
        sorted!(Solution::nums_same_consec_diff(3, 9))
    );
}
