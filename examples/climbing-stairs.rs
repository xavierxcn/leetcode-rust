use std::collections::HashMap;

fn main() {
    let r = Solution::climb_stairs(7);
    println!("{:?}", r)
}


struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = HashMap::new();
        dp.insert(0, 1);
        dp.insert(1, 1);
        for i in 2..=n {
            dp.insert(i, dp.get(&(i - 1)).unwrap() + dp.get(&(i - 2)).unwrap());
        }
        *dp.get(&n).unwrap()
    }
}