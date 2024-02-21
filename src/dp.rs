use core::num;
use std::borrow::Borrow;
use std::cmp::{self, min};

struct Solution;

// **************************************** 0-1背包 ****************************************
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if target.abs() > sum {
            return 0;
        }
        if (target + sum) % 2 == 1 {
            return 0;
        }
        let size = (sum + target) as usize / 2;
        let mut dp = vec![0; size + 1];
        dp[0] = 1;
        for n in nums {
            for s in (n as usize..=size).rev() {
                dp[s] += dp[s - n as usize];
            }
        }
        dp[size]
    }
}

#[test]
fn test_494() {
    let test_vec = vec![1, 0];
    println!("result={}", Solution::find_target_sum_ways(test_vec, 1));
}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![usize::default(); n as usize + 1]; m as usize + 1];
        // dp[i][j] 最多 有 i 个 0 和 j 个 1 时, 子集的最大长度
        for str_i in strs.iter() {
            let zero_count = str_i.as_str().chars().filter(|&item| item == '0').count();
            let one_count = str_i.len() - zero_count;
            for m_idx in (zero_count..=m as usize).rev() {
                for n_idx in (one_count..=n as usize).rev() {
                    dp[m_idx][n_idx] = std::cmp::max(
                        dp[m_idx][n_idx],
                        dp[m_idx - zero_count][n_idx - one_count] + 1,
                    );
                }
            }
        }
        dp[m as usize][n as usize] as i32
    }
}
#[test]
fn test_474() {
    let strs = vec!["10", "0", "1"];
    let vec_string: Vec<String> = strs.iter().map(|&item| item.to_string()).collect();
    println!("result={}", Solution::find_max_form(vec_string, 1, 1));
}

// **************************************** 完全背包 ****************************************
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![usize::default(); (amount + 1) as usize];
        dp[0] = 1;
        // dp[i]表示面额为i的凑数方法
        for coin in coins {
            if coin > amount {
                continue;
            }
            for count in coin..=amount {
                dp[count as usize] += dp[(count - coin) as usize];
            }
        }

        (*dp.last().unwrap()) as i32
    }
}

#[test]
fn test_518() {
    let coins = vec![1, 2, 5];
    println!("result={}", Solution::change(5, coins));
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![usize::default(); target as usize + 1];
        dp[0] = 1;

        for count in 1..=target {
            for num in nums.iter() {
                if *num > count {
                    continue;
                }
                dp[count as usize] += dp[(count - num) as usize];
            }
        }

        (*dp.last().unwrap()) as i32
    }
}

#[test]
fn test_377() {
    let nums = vec![1, 2, 3];
    println!("result={}", Solution::combination_sum4(nums, 4));
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use core::cmp::min;
        let amount = amount as usize;
        let mut dp = vec![i32::MAX as usize; amount + 1];
        dp[0] = 0;

        for coin in coins {
            for count in coin as usize..=amount {
                dp[count] = min(dp[count], dp[count - coin as usize] + 1);
            }
        }
        if *dp.last().unwrap() >= i32::MAX as usize {
            -1
        } else {
            *dp.last().unwrap() as i32
        }
    }
}

#[test]
fn test_322() {
    let nums = vec![1];
    println!("result={}", Solution::coin_change(nums, 0));
}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        use core::cmp::min;
        let n = n as usize;
        let max_num = (n as f32).sqrt() as usize;

        let mut dp = vec![i32::MAX as usize; n + 1];
        dp[0] = 0;

        for num in 1..=max_num {
            for sum in num * num..=n {
                dp[sum] = min(dp[sum], dp[sum - num * num] + 1);
            }
        }

        *dp.last().unwrap() as i32
    }
}

#[test]
fn test_279() {
    println!("result={}", Solution::num_squares(12));
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        // dp[i]: 考虑s[0..i]的子串能否由word_dict构成
        dp[0] = true;

        for len_ in 1..=s.len() {
            for word in word_dict.iter() {
                // 考虑一定长度的字符串
                if len_ < word.len() {
                    continue;
                }
                let prev_len = len_ - word.len();

                let cmp_str = s.get(prev_len..len_).unwrap();
                if cmp_str.eq(word) && !dp[len_] {
                    dp[len_] = dp[prev_len];
                }
            }
        }

        dp[s.len()]
    }
}

#[test]
fn test_139() {
    let s = "dogs".to_string();
    let wordDict = vec!["dog", "s", "gs"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("result={}", Solution::word_break(s, wordDict));
}

impl Solution {
    pub fn rob1(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut dp = vec![usize::default(); nums.len() + 1];
        // dp[i]: 考虑到地i个(从1开始)房屋时, 能偷窃的金额的上限
        dp[1] = nums[0] as usize;
        for (idx, num) in nums.iter().enumerate() {
            if idx < 1 {
                continue;
            }
            dp[idx + 1] = max(dp[idx], dp[idx - 1] + *num as usize);
        }
        *dp.last().unwrap() as i32
    }
}

// 打家劫舍1
#[test]
fn test_198() {
    let vec = vec![2, 7, 9, 3, 1];
    println!("result={}", Solution::rob1(vec));
}

impl Solution {
    pub fn rob_no_ring(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut dp = vec![usize::default(); nums.len() + 1];
        // dp[i]: 考虑到地i个(从1开始)房屋时, 能偷窃的金额的上限
        dp[1] = nums[0] as usize;
        for (idx, num) in nums.iter().enumerate() {
            if idx < 1 {
                continue;
            }
            dp[idx + 1] = max(dp[idx], dp[idx - 1] + *num as usize);
        }
        *dp.last().unwrap() as i32
    }

    pub fn rob2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let no_first_vec = nums[1..].to_vec();
        let no_last_vec = nums[..nums.len() - 1].to_vec();
        let sum_1 = Solution::rob_no_ring(no_first_vec);
        let sum_2 = Solution::rob_no_ring(no_last_vec);
        sum_1.max(sum_2)
    }
}

// 337: 打家劫舍3
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
// impl Solution {
//     fn tranverse(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
//         match root {
//             None => (0, 0),
//             Some(inner) => {
//                 let l_res = Solution::tranverse(&inner.borrow().left);
//                 let r_res = Solution::tranverse(&inner.borrow().right);

//                 (
//                     l_res.0.max(l_res.1) + r_res.0.max(r_res.1),
//                     inner.borrow().val + l_res.0 + r_res.0,
//                 )
//             }
//         }
//     }
//     pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let ans = Solution::tranverse(&root);
//         ans.0.max(ans.1)
//     }
// }
