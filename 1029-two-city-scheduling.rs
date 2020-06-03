struct Solution;

// https://leetcode.com/problems/two-city-scheduling/discuss/278898/Java-2ms-sorting-solution-with-explanation
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_unstable_by(|a, b| (a[1] - a[0]).cmp(&(b[1] - b[0])));
        let mut min_cost = 0;
        for i in 0..costs.len() / 2 {
            min_cost += costs[i][1];
        }
        for i in costs.len() / 2..costs.len() {
            min_cost += costs[i][0];
        }
        min_cost
    }
}

fn main() {
    assert_eq!(
        110,
        Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ])
    );
}
