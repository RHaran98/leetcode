struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let (s1, s2) = (word1.len(), word2.len());
        let mut ans = String::new();
        for (c1, c2) in word1.chars().zip(word2.chars()) {
            ans.push(c1);
            ans.push(c2);
        }
        if s1 > s2 {
            ans.push_str(&word1[s2..s1]);
        } else if s2 > s1 {
            ans.push_str(&word2[s1..s2]);
        }

        ans
    }
}
fn main() {
    let inp1 = "abcd".to_string();
    println!("Inp1 {:?}", inp1);

    let inp2 = "tsu".to_string();
    println!("Inp2 {:?}", inp2);

    let s = Solution::merge_alternately(inp1, inp2);
    println!("Ans {:?}", s);
}
