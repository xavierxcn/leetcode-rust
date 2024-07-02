use std::collections::HashSet;

fn main() {

}

fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let mut prime_map:HashSet<i32> = HashSet::new();
    let mut max = 0;
    let mut min = nums.len();
    for i in 0..nums.len() {
        if prime_map.contains(&nums[i]) || is_prime_difference(nums[i]) {
            prime_map.insert(nums[i]);
            min = i;
            break
        }
    }

    for i in 0..nums.len() {
        let j = nums.len() - i - 1;
        if prime_map.contains(&nums[j]) || is_prime_difference(nums[j]) {
            prime_map.insert(nums[j]);
            max = j;
            break
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
