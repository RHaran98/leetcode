use std::cmp::{max, min};

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // First solution
        // let size = prices.len();
        // let mut prefix_array = vec![0; prices.len()];
        // let mut postfix_array = vec![0; prices.len()];
        // // let (mut min, mut max) = (prices[0], prices[size-1]);
        // prefix_array[0] = prices[0];
        // postfix_array[size-1] = prices[size-1];

        // for i in 1..size {
        //     prefix_array[i] = min(prices[i],prefix_array[i-1]);
        //     postfix_array[size- 1 - i] = max(postfix_array[size-i],prices[size - i-1]);
        // }
        // let mut ans = 0;
        // // println!("Prefix: {:?}",prefix_array);
        // // println!("Postfix: {:?}",postfix_array);
        // for i in 0..size {
        //     ans = max(postfix_array[i] - prefix_array[i],ans);
        // }
        // ans

        let size = prices.len();
        let mut min_price_so_far = prices[0];
        let mut max_profit = 0;
        for i in 1..size {
            min_price_so_far = min(min_price_so_far, prices[i]);
            max_profit = max(max_profit, prices[i] - min_price_so_far);
        }
        max_profit
    }
}

fn main() {
    let inp = vec![8, 7, 2, 1];
    println!("Inp {:?}", inp);

    let s = Solution::max_profit(inp);
    println!("Ans {:?}", s);
}
