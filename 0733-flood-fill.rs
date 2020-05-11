struct Solution;

type Image = Vec<Vec<i32>>;
impl Solution {
    pub fn flood_fill(image: Image, sr: i32, sc: i32, new_color: i32) -> Image {
        let sr = sr as usize;
        let sc = sc as usize;

        let mut image = image;
        let color = image[sr][sc];
        if color != new_color {
            Self::dfs(&mut image, sr, sc, color, new_color);
        }
        image
    }

    fn dfs(image: &mut Image, r: usize, c: usize, color: i32, new_color: i32) {
        if image[r][c] == color {
            image[r][c] = new_color;
            if r >= 1 {
                Self::dfs(image, r - 1, c, color, new_color);
            }
            if c >= 1 {
                Self::dfs(image, r, c - 1, color, new_color);
            }
            if r + 1 < image.len() {
                Self::dfs(image, r + 1, c, color, new_color);
            }
            if c + 1 < image[0].len() {
                Self::dfs(image, r, c + 1, color, new_color);
            }
        }
    }
}

fn main() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    let result = Solution::flood_fill(image, 1, 1, 2);
    assert_eq!(result, expected);
}
