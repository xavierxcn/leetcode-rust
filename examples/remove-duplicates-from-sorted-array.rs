
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/
fn main() {

}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1
        }
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if i < j && nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }

            j += 1
        }


        (i + 1) as i32
    }
}