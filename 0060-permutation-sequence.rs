struct Solution;

const FACTORIAL: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k - 1; // 0 indexed
        let mut numbers = Vec::with_capacity(n as usize);
        for i in 1..=n as u32 {
            numbers.push(i);
        }

        let n = n as usize;
        let mut result = String::with_capacity(n);
        for i in (0..n).rev() {
            // get the index of current digit
            let idx = (k / FACTORIAL[i]) as usize;
            k = k % FACTORIAL[i];
            result.push(std::char::from_digit(numbers[idx], 10).unwrap());
            numbers.remove(idx);
        }
        result
    }
}

fn main() {
    assert_eq!("1", Solution::get_permutation(1, 1));
    assert_eq!("321", Solution::get_permutation(3, 6));
    assert_eq!("213", Solution::get_permutation(3, 3));
    assert_eq!("2314", Solution::get_permutation(4, 9));
}
