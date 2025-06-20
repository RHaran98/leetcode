struct Solution;
// First soln
// impl Solution {
//     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//         nums.sort_unstable();
//         let mut ans: HashSet<Vec<i32>> = HashSet::new();
//         let l = nums.len();
//         let mut map: HashSet<i32>  = HashSet::new();
//         map.insert(nums[0]);
//         for j in 1..l-1 {
//             for k in j+1..l {
//                 let val = -nums[j] - nums[k];
//                 match map.contains(&val) {
//                     true => {
//                         let mut temp = vec![val,nums[j],nums[k]];
//                         temp.sort_unstable();
//                         ans.insert(temp);
//                     },
//                     _ => (),
//                 }

//             }
//             map.insert(nums[j]);
//         }

//         ans.into_iter().collect()

//     }
// Second solution
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let size = nums.len();
        let mut ans = Vec::with_capacity(size/3);
        // println!("Sort {:?}",nums);

        for i in 0..size - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                // println!("Skipping i -> {}",i);
                continue;
            }
            let target = -nums[i];
            let (mut j, mut k) = (i + 1, size - 1);
            while j < k {
                // println!("{:?} -> {:?}", vec![i,j,k], vec![nums[i],nums[j],nums[k]]);
                let comp = nums[j] + nums[k];
                if comp < target {
                    j += 1;
                } else if comp > target {
                    k -= 1;
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    while j < k && nums[k - 1] == nums[k] {
                        k -= 1;
                    }

                    j += 1;
                    k -= 1;
                }
            }
        }
        ans
    }
}
fn main() {
    let inp = vec![-1, 0, 1, 2, -1, -4];
    println!("Inp {:?}", inp);

    let s = Solution::three_sum(inp);
    println!("Ans {:?}", s);
}
