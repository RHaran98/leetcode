use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_map: HashSet<i32> = nums.into_iter().collect();
        let mut current_max = 0;
        while let Some(item) = num_map.iter().next().cloned() {
            let mut size = 0;
            for i in item.. {
                match num_map.remove(&i) {
                    true => {
                        size += 1;
                    }
                    false => {
                        break;
                    }
                }
            }
            for i in std::iter::successors(Some(item - 1), |&x| Some(x - 1)) {
                match num_map.remove(&i) {
                    true => {
                        size += 1;
                    }
                    false => {
                        break;
                    }
                }
            }
            if size > current_max {
                current_max = size;
            }
        }

        current_max
    }
}
fn main() {
    let inp = vec![1, 2, 3, 0, 10];
    println!("{:?}", inp);

    let s = Solution::longest_consecutive(inp);
    println!("{:?}", s);
}
