

// https://leetcode.cn/problems/move-zeroes/description/
fn main() {

}

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut l = 0;
    let mut r = 0;
    while r < nums.len() {
        if nums[r] != 0 {
            let tmp = nums[r];
            nums[r] = nums[l];
            nums[l] = tmp;
            l+=1;
        }
        r+=1;
    }

}