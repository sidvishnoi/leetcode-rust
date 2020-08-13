struct CombinationIterator {
    chars: Vec<char>,
    masks: Vec<i32>,
    combination_length: usize,
    ptr: usize,
}

impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        Self {
            chars: characters.chars().collect(),
            masks: Self::get_masks(characters.len() as i32, combination_length),
            combination_length: combination_length as usize,
            ptr: 0usize,
        }
    }

    pub fn next(&mut self) -> String {
        let mut mask = self.masks[self.ptr];
        self.ptr += 1;

        let mut result = Vec::with_capacity(self.combination_length);
        let mut index = self.chars.len();
        while mask > 0 && index > 0 {
            index -= 1;
            if mask & 1 > 0 {
                result.push(self.chars[index]);
            }
            mask >>= 1;
        }
        result.into_iter().rev().collect()
    }

    pub fn has_next(&self) -> bool {
        self.ptr < self.masks.len()
    }

    fn get_masks(max_range: i32, num_ones: i32) -> Vec<i32> {
        let mut masks = Vec::new();
        let n: i32 = (1 << max_range) - 1;
        for i in (0..n).rev() {
            if i.count_ones() as i32 == num_ones {
                masks.push(i);
            }
        }
        masks
    }
}

fn main() {
    let mut iterator = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!("ab".to_string(), iterator.next());
    assert_eq!(true, iterator.has_next());
    assert_eq!("ac".to_string(), iterator.next());
    assert_eq!(true, iterator.has_next());
    assert_eq!("bc".to_string(), iterator.next());
    assert_eq!(false, iterator.has_next());
}
