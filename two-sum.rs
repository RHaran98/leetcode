pub fn argsort<T: Ord>(slice: &[T]) -> Vec<usize> {
    let n = slice.len();
    let mut keys : Vec<_> = (0..n).collect();
    keys.sort_by_key(|x| &slice[*x]);
    keys
}

impl Solution {
    pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        // nums.sort();
        let indices = argsort(&nums);
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i!=j {
            if (nums[indices[i]] + nums[indices[j]] < target)
                {i += 1;}
            else if (nums[indices[i]] + nums[indices[j]] > target)
                {j -= 1;}
            else
                {return vec![(indices[i] as i32),(indices[j] as i32)]}
        }

        vec![1,1]


    }
}