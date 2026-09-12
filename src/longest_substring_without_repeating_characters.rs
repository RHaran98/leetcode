use std::cmp::max;
const N_CHARS: usize = b'~' as usize - b' ' as usize + 1;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let char_index = |x: u8| (x - b' ') as usize;
        let mut curr_counts: [Option<usize>; N_CHARS] = [None; N_CHARS];
        let chars: &[u8] = s.as_bytes();
        let size = chars.len();
        if size == 0 {
            return 0;
        }
        let mut i = 0;
        curr_counts[char_index(chars[0])] = Some(0);
        let mut max_substring: i32 = 1;
        for j in 1..size {
            if let Some(index) = curr_counts[char_index(chars[j])] {
                i = max(index, i);
            }

            curr_counts[char_index(chars[j])] = Some(j);
            max_substring = max(max_substring, (j - i + 1) as i32);
            // println!("i,j,max: {} {} {}",i,j,max_substring);
        }
        max_substring
    }
}
fn main() {
    let inp = "abcdec".to_string();
    println!("Inp {:?}", inp);

    let s = Solution::length_of_longest_substring(inp);
    println!("Ans {:?}", s);
}
