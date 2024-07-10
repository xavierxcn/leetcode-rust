// https://leetcode.cn/problems/container-with-most-water/description/

use std::cmp::{max, min};

fn main() {

}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut left, mut right) = (0, height.len() as i32 - 1);

        while left < right {
            let area = min(height[left as usize], height[right as usize]) * (right - left);
            max_area = max(max_area, area);

            if height[left as usize] < height[right as usize] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}