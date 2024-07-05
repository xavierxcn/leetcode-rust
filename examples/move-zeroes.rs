

// https://leetcode.cn/problems/move-zeroes/description/
fn main() {

}

fn move_zeroes(nums: &mut Vec<i32>) {
    let l = nums.len();
    nums.retain(|&x| x != 0);
    nums.resize(l, 0);
}