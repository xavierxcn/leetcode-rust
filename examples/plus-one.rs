
// https://leetcode.cn/problems/plus-one/description/
fn main() {
    let digits = vec![9, 9, 9];
    let s = Solution::plus_one(digits);
    println!("{:?}", s);

}

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len()-1;
        let mut ascende = true;
        let mut rdata = vec![0;digits.len()+1];
        while  i >= 0 {
            let mut v = digits[i];
            if ascende {
                v += 1;
            }
            if v >= 10 {
                v = 0;
                ascende = true;
            } else {
                ascende  = false
            }

            rdata[i+1] = v;
            if i == 0 {
                break
            }
            i -= 1
        }

        if ascende {
            rdata[0] = 1
        } else {
            rdata = Vec::from(&rdata[1..rdata.len()])
        }

        rdata
    }
}