
// https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    println!("{}", Solution::find_median_sorted_arrays(nums1, nums2))

}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i1 = 0;
        let mut i2 = 0;
        let mut nums = Vec::new();
        while i1 < nums1.len() || i2 < nums2.len() {
            if i1 >= nums1.len() {
                nums.push(nums2[i2]);
                i2 += 1;
                continue
            }

            if i2 >= nums2.len() {
                nums.push(nums1[i1]);
                i1 += 1;
                continue
            }

            if nums1[i1] < nums2[i2] {
                nums.push(nums1[i1]);
                i1 += 1;
                continue
            } else {
                nums.push(nums2[i2]);
                i2 += 1;
                continue
            }
        };

        if nums.len() % 2 == 1 {
            return nums[nums.len() / 2] as f64;
        } else {
            return ((nums[nums.len()/2] + nums[nums.len()/2-1]) as f64) / 2.0
        }
    }
}
