struct Solution;
impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        s.retain(|c| c.is_alphanumeric());
        s.make_ascii_lowercase();
        let c: Vec<char> = s.chars().collect();

        let size = c.len();
        for i in 0..size {
            if c[i] != c[size - i - 1] {
                return false;
            }
        }
        true
    }
}
fn main() {
    let inp = "Hell ol leh".to_string();
    println!("{:?}", inp);

    let s = Solution::is_palindrome(inp);
    println!("{:?}", s);
}
