struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as usize;

        let mut distribution = vec![0; num_people];
        let mut current_candy_count = 1;
        while candies > 0 {
            for i in 0..num_people {
                if candies >= current_candy_count {
                    distribution[i] += current_candy_count;
                    candies -= current_candy_count;
                    current_candy_count += 1;
                } else {
                    distribution[i] += candies;
                    candies = 0;
                    break;
                }
            }
        }
        distribution
    }
}

fn main() {
    assert_eq!(vec![1, 2, 3, 1], Solution::distribute_candies(7, 4));
    assert_eq!(vec![5, 2, 3], Solution::distribute_candies(10, 3));
}
