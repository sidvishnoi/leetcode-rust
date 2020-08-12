struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = Vec::with_capacity((row_index + 1) as usize);
        let mut num: i64 = 1;
        let row_index = row_index as i64;
        for j in 0..=row_index {
            row.push(num as i32);
            num = num * (row_index - j) / (j + 1);
        }
        row
    }
}

fn main() {
    assert_eq!(vec![1], Solution::get_row(0));
    assert_eq!(vec![1, 1], Solution::get_row(1));
    assert_eq!(vec![1, 2, 1], Solution::get_row(2));
    assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
    assert_eq!(
        vec![
            1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
            193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110,
            1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100, 13884156,
            4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
        ],
        Solution::get_row(33)
    );
}