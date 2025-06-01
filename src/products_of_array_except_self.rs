struct Solution;
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        // Solution 1:
        // let mut prefix = vec![]; nums.iter().fold(1,|acc,x| {let prod = acc*x ;prefix.push(prod);prod});
        // let mut suffix = vec![]; nums.iter().rev().fold(1,|acc,x| {let prod = acc*x ;suffix.push(prod);prod}); suffix = suffix.into_iter().rev().collect();
        // println!("prefix {:?}",prefix);
        // println!("suffix {:?}",suffix);
        // let l = nums.len();
        // nums[0] = suffix[1];
        // nums[l-1] = prefix[l-2];
        // for i in 1..l-1 {
        //     nums[i] = prefix[i-1]*suffix[i+1];
        // }
        // nums

        // Solution 2
        let size = nums.len();
        let mut prefix = vec![1];

        for i in 1..size {
            prefix.push(prefix[i - 1] * nums[i - 1]);
        }

        let mut temp = nums[size - 1];
        for i in (0..size - 1).rev() {
            prefix[i] = prefix[i] * temp;
            temp *= nums[i];
        }
        prefix
    }
}

fn main() {
    let inp = vec![1, 2, 3, 0, 10];
    println!("{:?}", inp);

    let s = Solution::product_except_self(inp);
    println!("{:?}", s);
}
