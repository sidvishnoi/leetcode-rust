pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0 as i32;
    }

    let mut l: usize = 0;
    for r in 0..nums.len() {
        if nums[r] != nums[l] {
            l += 1;
            nums[l] = nums[r];
        }
    }
    l += 1;
    l as i32
}

#[cfg(test)]
mod remove_duplicates {
    use super::remove_duplicates;

    #[test]
    pub fn no_duplicates() {
        let mut nums = vec![3, 5, 7, 9];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 4);
        let expected = vec![3, 5, 7, 9];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
    #[test]
    pub fn some_are_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 5);
        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
    #[test]
    pub fn all_are_duplicates() {
        let mut nums = vec![3, 3, 3, 4, 4, 4];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 2);
        let expected = vec![3, 4];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
    #[test]
    pub fn empty_input() {
        let mut nums = vec![];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 0);
        let expected = vec![];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
}
