// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
use std::collections::HashMap;

fn main() {

}

fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ret = 0;
    let mut m = HashMap::new();
    let mut start = 0;
    for i in 0..s.len() {
        if m.contains_key(&s[i]) {
            start = start.max(m[&s[i]] + 1);
        }
        m.insert(s[i], i);
        ret = ret.max(i - start + 1);
    }
    ret as i32
}