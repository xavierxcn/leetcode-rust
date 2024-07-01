// https://leetcode.cn/problems/two-sum/description/

use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ret = two_sum(nums, target);
    println!("{:?}", ret);

}

// 两数只和，利用哈希表缓存计算结果，在哈希表中匹配
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 返回值
    let mut ret = Vec::new();
    // 哈希表缓存计算结果，key: 计算结果，value: 下标
    let mut sum = HashMap::new();
    for i in 0..nums.len() {
        let v = nums[i];
        if sum.contains_key(&v) {
            // 如果匹配到，说明 v = target - nums[i]
            ret.push(sum[&v]);
            ret.push(i as i32);
            return ret;
        }

        // 在这里缓存计算结果
        sum.insert(target - v, i as i32);
    }
    Vec::new()
}