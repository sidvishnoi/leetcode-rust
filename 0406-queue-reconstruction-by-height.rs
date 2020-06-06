struct Solution;

// [[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]
// sorted:
// [[7, 0], [7, 1], [6, 1], [5, 0], [5, 2], [4, 4]]
//
// [[7, 0]]
// [[7, 0], [7, 1]]
// [[7, 0], [6, 1], [7, 1]]
// [[5, 0], [7, 0], [6, 1], [7, 1]]
// [[5, 0], [7, 0], [5, 2], [6, 1], [7, 1]]
// [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
//
// understanding output:
// [0]: there are 0 people with h >= 5 in front of [0]
// [1]: there are 0 people with h >= 7 in front of [1]
// [2]: there are 2 people with h >= 5 in front of [2] (that is, people with height 5 and 7)
// [3]: there are 1 people with h >= 6 in front of [3] (that is, people with height 7)
// [4]: there are 5 people with h >= 4 in front of [4] (that is, people with height 5, 7, 5 and 6)
// [5]: there are 1 people with h >= 7 in front of [5] (that is, people with height 7 (at [1]))
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;

        // sort by decreasing height, then increasing k
        people.sort_by(|a, b| a[0].cmp(&b[0]).reverse().then(a[1].cmp(&b[1])));
        let mut result = Vec::with_capacity(people.len());
        for p in people {
            let k = p[1] as usize;
            result.insert(k, p);
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1]
        ],
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ])
    );
}
