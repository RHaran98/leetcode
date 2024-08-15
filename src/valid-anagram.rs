use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut check = HashMap::<char, i32>::new();
        for c in s.chars() {
            let cnt = check.entry(c).or_insert(0);
            *cnt += 1;
        }

        for c in t.chars() {
            match check.get_mut(&c) {
                Some(cnt) => {
                    *cnt -= 1;
                    if *cnt == 0 {
                        check.remove(&c);
                    }
                },
                None => return false
            }
        }
        if check.is_empty() {
            return true
        }
        false                
    }
}