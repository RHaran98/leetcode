use std::cmp::max;
const N_CHARS: usize = b'~' as usize - b' ' as usize + 1;

struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {

    }
}

fn main() {
    let inp = "abcdec".to_string();
    println!("Inp {:?}", inp);

    let s = Solution::length_of_longest_substring(inp);
    println!("Ans {:?}", s);
}
