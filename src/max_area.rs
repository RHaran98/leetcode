use std::cmp;

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len()-1);
        let mut max_area = 0;
        while i < j {
            let area = (j-i) as i32 * cmp::min(height[i],height[j]);
            // println!("{:?} -> {:?}", (i,j), (height[i], height[j]) );
            max_area = cmp::max(area, max_area);
            if height[i]<height[j] {
                i+= 1;
            }
            else {
                j-= 1;
            }
        }
        max_area
    }
}
fn main() {
    let inp = vec![8,7,2,1];
    println!("Inp {:?}", inp);

    let s = Solution::max_area(inp);
    println!("Ans {:?}", s);
}
