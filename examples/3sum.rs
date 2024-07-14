fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    println!("{:?}", Solution::three_sum(nums));
}

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // a + b = -c
        let mut ret = Vec::new();
        nums.sort();
        for t in 0..nums.len()-2 {
            if t > 0 && nums[t] == nums[t-1] {
                continue
            }
            let mut i = t+1;
            let mut j= nums.len()-1;
            while i < j {
                let s = nums[t] + nums[i] + nums[j];
                if s < 0 {
                    i += 1;
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                } else if s > 0 {
                    j -= 1;
                    while i < j && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                } else {
                    ret.push(vec![nums[t], nums[i], nums[j]]);
                    i += 1;
                    j -= 1;
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                    while i < j && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                }
            }
        }

        ret

    }
}