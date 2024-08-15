use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let freqs = nums.into_iter().fold(HashMap::<i32,u8>::new(), |mut map, num| {
            map.entry(num).and_modify(|x| *x+=1 ).or_insert(1);
            map
        });
        let mut vals: Vec<(i32, u8)> = freqs.into_iter().collect();
        vals.sort_unstable_by_key(|x|std::u8::MAX - x.1);
        vals.iter().take(k as usize).map(|x| x.0).collect()
    }
}

fn main()  {
    let s = Solution::top_k_frequent(vec![1,1,2,3,4,4,3,3,2,5],2);
    println!("{:?}",s);
}