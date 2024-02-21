use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn getBitSqureSum(n: i32) -> i32 {
        let mut sum = 0;
        let mut m = n;
        while m > 0 {
            sum += (m % 10).pow(2);
            m /= 10;
        }
        return sum;
    }
    pub fn is_happy(n: i32) -> bool {
        let mut m = n;
        let mut map: HashSet<i32> = HashSet::new();

        while !map.contains(&m) {
            map.insert(m);
            m = Solution::getBitSqureSum(m);
            if m == 1 {
                return true;
            }
        }

        false
    }
}

#[test]
fn test_202() {
    println!("result={}", Solution::is_happy(2));
}
