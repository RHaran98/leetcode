use std::collections::HashMap;
const N_LETTERS: usize = 26;

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let freqs = nums.fold(HasMap::<i32,u8>::new(), |mut map, num| {
            
        } );
    }
}

fn main()  {
    let s = Solution::tok_k_frequent(vec![1,1,2,3,4,4,3,3,2,5],2);
    println!("JA");
    println!("{:?}",s);
}