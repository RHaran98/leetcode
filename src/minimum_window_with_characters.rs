struct Solution;

use std::cmp::max;
const N_CHARS: usize = b'~' as usize - b' ' as usize + 1;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut i = 0;
        let mut j = 0;
        let mut min_so_far = usize::MIN;
        let mut min_i = 0;
        let mut min_j = 0;
        let chars = s.chars();
        let size:usize = chars.count();
        let mut char_map: [i32;N_CHARS] = [0;N_CHARS];
        let char_index = |x:u8| x as usize - b'a' as usize;

        loop {
            if j+1 >= size || i >= size {
                break;
            }

            if char_map.all(|x| x>= 0) {
                if (j-i+i) < min_so_far 
                i+=1;
                char_map[ char_index(chars.nth(i-1).unwrap() as u8) ] -= 1;
            }
            else {
                j+=1;
                char_map[ char_index(chars.nth(j).unwrap() as u8) ] += 1;
            }
        
            
        }
        stringify!(1).to_string()
    }   
}

fn main() {
    let inp = "abcdec".to_string();
    println!("Inp {:?}", inp);

    let s = Solution::length_of_longest_substring(inp);
    println!("Ans {:?}", s);
}
