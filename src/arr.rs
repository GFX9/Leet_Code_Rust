struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        use std::cmp::min;

        let mut begin = 0;

        let mut sum = 0;

        let mut len = i32::MAX;

        for end in begin..nums.len() {
            sum += nums[end];
            while sum >= target {
                len = min((end - begin + 1) as i32, len);

                sum -= nums[begin];
                begin += 1;
            }
        }
        
        if len == i32::MAX {
            0
        } else {
            len
        }
    }
}

#[test]
fn test_209() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("result={}", Solution::min_sub_array_len(11, nums));
}
