struct Solution;

// Two steps to bubble largest values to the end of list to sort:
//  1. Bring value_to_sort to front of list by flipping list until it,
//  2. Move it to the end of list by flipping rest of the list.
//
// Example: [2,1,{4},3]
//  -> {4},1,2,3 -> [{3},2,1],4
//  -> [1,2],{3},4
impl Solution {
    pub fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let largest_value = a.len() as i32;
        for value_to_sort in (1..=largest_value).rev() {
            let idx = a.iter().position(|&v| v == value_to_sort).unwrap() + 1;

            if idx as i32 == value_to_sort {
                // value_to_sort is in right place already
                continue;
            }

            if idx != 1 {
                // not in front of list already
                result.push(idx as i32);
                Self::flip(&mut a, idx);
            }

            result.push(value_to_sort);
            Self::flip(&mut a, value_to_sort as usize);
        }

        result
    }

    fn flip(a: &mut Vec<i32>, k: usize) {
        let mut i = 0;
        let mut j = k - 1;
        while i < j {
            a.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

fn main() {
    assert_eq!(
        vec![3, 4, 2, 3, 2],
        Solution::pancake_sort(vec![3, 2, 4, 1])
    );
    assert_eq!(Vec::<i32>::new(), Solution::pancake_sort(vec![1, 2, 3]));
}
