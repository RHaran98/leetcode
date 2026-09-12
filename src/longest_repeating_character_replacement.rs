struct Solution;
use std::{cmp::max, cmp::min, println};
const N_CHARS: usize = b'Z' as usize - b'A' as usize + 1;
impl Solution {
    pub fn get_max_char(char_counts: [i32; N_CHARS], k: i32, i: usize, j: usize) -> i32 {
        let sum_max = char_counts
            .iter()
            .fold((0, i32::MIN), |init, x| (init.0 + x, max(*x, init.1)));
        let summer =sum_max.1 + min(sum_max.0 - sum_max.1, min(k, (j - i + 1) as i32));
        if summer  == (j-i+1) as i32 {summer} else {0}
    }
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if k>=s.len() as i32 {
            return  s.len() as i32;
        }
        let s = s.as_bytes();
        let char_index = |x: u8| x as usize - b'A' as usize;
        let mut char_counts = [0; N_CHARS];
        let mut max_freq = 0;
        let mut max_str = 0;
        let mut prev_max_str = i32::MIN;
        char_counts[char_index(s[0])] = 1;

        let (mut i, mut j) = (0, 0);
        loop {
            if (i + 1   > s.len() - (k as usize) || j + 1 > s.len()) {
                break;
            } 
            // else {
            //     println!("cont");
            // }
            // let idx = char_index(s[j]);

            // max_str = max(char_counts[idx].0,max_freq);
            let max_candidate = Self::get_max_char(char_counts, k, i, j);
            max_str = max(max_str, max_candidate);
            // println!(
            //     "i:{} j:{} val_i {} val_j {} curr_max {} max_str {}, {} {}",
            //     i, j, s[i], s[j], max_candidate, max_str, char_counts[0], char_counts[1]
            // );
            if (max_candidate > prev_max_str) {
                j = j + 1;
                if j + 1 <= s.len() {
                    char_counts[char_index(s[j])] += 1;
                }
            } 
            else {
                if (i + 1 != j) {
                    char_counts[char_index(s[i])] -= 1;
                    i = i + 1;
                } else {
                    j += 1;
                    if (j + 1 <= s.len()) {
                        char_counts[char_index(s[j])] += 1;
                    }
                }
            }
            prev_max_str = max_candidate;
        }

        max_str
    }
}
fn main() {
    let inp = String::from("AAAAA");
    println!("Inp {:?}", inp);

    let s = Solution::character_replacement(inp, 5);
    println!("Ans {:?}", s);
}
