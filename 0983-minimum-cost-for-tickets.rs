struct Solution;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

impl Solution {
    pub fn mincost_tickets(travel_days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let (cost_day, cost_week, cost_month) = (costs[0], costs[1], costs[2]);

        let mut dp = vec![0; travel_days.len() + 1];
        let mut week = travel_days.len() - 1;
        let mut month = travel_days.len() - 1;
        for day in (0..travel_days.len()).rev() {
            while travel_days[week] - travel_days[day] >= 7 {
                week -= 1;
            }
            while travel_days[month] - travel_days[day] >= 30 {
                month -= 1;
            }
            dp[day] = min!(
                cost_day + dp[day + 1],
                cost_week + dp[week + 1],
                cost_month + dp[month + 1]
            );
        }
        dp[0]
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
