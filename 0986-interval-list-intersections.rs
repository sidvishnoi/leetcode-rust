struct Solution;

type Intervals = Vec<Vec<i32>>;
impl Solution {
    #[allow(non_snake_case)]
    pub fn interval_intersection(A: Intervals, B: Intervals) -> Intervals {
        let mut A = A.into_iter().peekable();
        let mut B = B.into_iter().peekable();
        let mut result = Vec::new();
        while let (Some(a), Some(b)) = (A.peek(), B.peek()) {
            let (a_start, a_end) = (a[0], a[1]);
            let (b_start, b_end) = (b[0], b[1]);
            // Check if a[i] intersects with b[j]
            // start and end are start and end points of intersection
            let start = std::cmp::max(a_start, b_start);
            let end = std::cmp::min(a_end, b_end);
            if start <= end {
                result.push(vec![start, end]);
            }

            // remove the interval with smallest end point
            if a_end < b_end {
                A.next();
            } else {
                B.next();
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25]
        ],
        Solution::interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        )
    );
}
