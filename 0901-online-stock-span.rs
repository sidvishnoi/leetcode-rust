#[derive(Default)]
struct StockSpanner {
    // price, num_elements_skipped
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut result = 1;
        while let Some((p, c)) = self.stack.pop() {
            if p <= price {
                result += c;
            } else {
                self.stack.push((p, c));
                break;
            }
        }
        self.stack.push((price, result));
        result
    }
}

fn main() {
    let mut stock_spanner = StockSpanner::new();
    assert_eq!(1, stock_spanner.next(100));
    assert_eq!(1, stock_spanner.next(80));
    assert_eq!(1, stock_spanner.next(60));
    assert_eq!(2, stock_spanner.next(70));
    assert_eq!(1, stock_spanner.next(60));
    assert_eq!(4, stock_spanner.next(75));
    assert_eq!(6, stock_spanner.next(85));
}
