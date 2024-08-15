use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut check = HashMap::<i32,bool>::new();
        for i in nums {
            match check.get(&i) {
                Some(_) => return true,
                None => {check.insert(i, true);},
            }
        }
        false
    }
}