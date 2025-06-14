use std::collections::HashMap;
const N_LETTERS: usize = 26;

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter().fold(HashMap::<[u8;N_LETTERS], Vec<String>>::new(), |mut map, s| {
            let freqs = s.bytes().fold( [0;N_LETTERS] , |mut a , c| {
                a[(c - b'a') as usize] += 1;
                a
            });
            map.entry(freqs).or_default().push(s);
            map
        } ).into_values().collect()
    }   
}   

fn main()  {
    let s = Solution::group_anagrams(vec!["abc".to_string(),"bac".to_string(),"ab".to_string(),"abcd".to_string(),"ba".to_string()]);
    println!("JA");
    println!("{:?}",s);
}