struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let s = s.as_bytes().to_vec();

        // Case: There are more open parenthesis but we have enough '*' so we can balance the parenthesis with ')'
        let mut left_balance = 0;
        for ch in s.iter() {
            if *ch == b')' {
                left_balance -= 1;
            } else {
                // ( or *
                left_balance += 1;
            }
            if left_balance < 0 {
                return false;
            }
        }

        // early exit: we already matched all parantheses
        if left_balance == 0 {
            return true;
        }

        // Case: There are more close parenthesis but we have enough '*' so we can balance the parenthesis with '('
        let mut right_balance = 0;
        for ch in s.into_iter().rev() {
            if ch == b'(' {
                right_balance -= 1;
            } else {
                // ) or *
                right_balance += 1;
            }
            if right_balance < 0 {
                return false;
            }
        }

        // Case: There are as many '(' than ')' so all parenthesis are balanced, we can ignore the extra '*'
        true
    }
}

fn main() {
    assert_eq!(true, Solution::check_valid_string("()".to_string()));
    assert_eq!(true, Solution::check_valid_string("(*)".to_string()));
    assert_eq!(true, Solution::check_valid_string("(*))".to_string()));
}
