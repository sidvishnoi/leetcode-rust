struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|num| {
                if num % 5 == 0 && num % 3 == 0 {
                    "FizzBuzz".to_string()
                } else if num % 3 == 0 {
                    "Fizz".to_string()
                } else if num % 5 == 0 {
                    "Buzz".to_string()
                } else {
                    num.to_string()
                }
            })
            .collect()
    }
}

fn main() {
    println!("{:#?}", Solution::fizz_buzz(16));
}
