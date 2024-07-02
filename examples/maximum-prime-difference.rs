use std::collections::HashMap;

fn main() {

}

fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let mut prime_map:HashMap<i32, bool> = HashMap::new();
    let mut max = 0;
    let mut min = nums.len();
    for i in 0..nums.len() {
        if prime_map.contains_key(&nums[i]) || is_prime_difference(nums[i]) {
            prime_map.insert(nums[i], true);
            if i > max {
                max = i
            }
            if i < min {
                min = i
            }
        }
    }

    return (max - min) as i32

}

fn is_prime_difference(num: i32) -> bool {
    if num < 2 {
        return false
    }

    if num == 2 {
        return true
    }

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
