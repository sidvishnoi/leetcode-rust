pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut left: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[left] = nums[i];
            left += 1;
        }
    }
    left as i32
}

#[cfg(test)]
mod remove_element {
    use super::remove_element;

    #[test]
    pub fn no_instance_of_val() {
        let mut nums = vec![3, 2, 2, 3];
        let len = remove_element(&mut nums, 0);
        assert_eq!(len, 4);
        let expected = vec![3, 2, 2, 3];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
    #[test]
    pub fn some_instances_of_val() {
        let mut nums = vec![3, 2, 2, 3];
        let len = remove_element(&mut nums, 3);
        assert_eq!(len, 2);
        let expected = vec![2, 2];
        assert_eq!(&nums[..len as usize], &expected[..]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 5);
        let expected = vec![0, 1, 3, 0, 4];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
    #[test]
    pub fn all_instances_of_val() {
        let mut nums = vec![3, 3, 3];
        let len = remove_element(&mut nums, 3);
        assert_eq!(len, 0);
        let expected = vec![];
        assert_eq!(&nums[..len as usize], &expected[..]);
    }
}
