struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let passes = [(1, costs[0]), (7, costs[1]), (30, costs[2])];
        let mut memo = vec![-1; days.len()];
        Self::find_cost(0, &days, &passes, &mut memo)
    }

    fn find_cost(
        day: usize,
        days: &Vec<i32>,
        passes: &[(i32, i32); 3],
        memo: &mut Vec<i32>,
    ) -> i32 {
        if day >= days.len() {
            return 0;
        }
        if memo[day] != -1 {
            return memo[day];
        }

        let mut result = std::i32::MAX;
        let mut d = day;
        for (validity_duration, pass_cost) in passes {
            while d < days.len() && days[d] < days[day] + validity_duration {
                d += 1;
            }
            result = std::cmp::min(result, Self::find_cost(d, days, passes, memo) + pass_cost);
        }
        memo[day] = result;
        result
    }
}

fn main() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    assert_eq!(11, Solution::mincost_tickets(days, costs));

    let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
    let costs = vec![2, 7, 15];
    assert_eq!(17, Solution::mincost_tickets(days, costs));
}
