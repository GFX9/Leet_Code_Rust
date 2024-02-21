struct Solution;

impl Solution {
    fn is_palindrome(s: &[char]) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let mut l_idx = 0;
        let mut r_idx = s.len() - 1;

        while l_idx <= r_idx {
            if s[l_idx] != s[r_idx] {
                return false;
            }
            l_idx += 1;
            r_idx -= 1;
        }
        true
    }
    fn backtrace(
        begin_idx: usize,
        s: &Vec<char>,
        rt: &mut Vec<Vec<String>>,
        rec: &mut Vec<String>,
    ) {
        if begin_idx >= s.len() {
            let rec_len = rec.iter().fold(0, |sum, item| sum + item.len());
            if rec_len == s.len() {
                rt.push(rec.clone());
            }
            return;
        }

        for end_idx in begin_idx..s.len() {
            let tmp_str: &[char] = &s[begin_idx..=end_idx];
            if !Solution::is_palindrome(tmp_str) {
                continue;
            }
            rec.push(tmp_str.iter().collect());
            Solution::backtrace(end_idx + 1, s, rt, rec);
            rec.pop();
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s_chars: Vec<char> = s.chars().collect();
        let mut rt: Vec<Vec<String>> = Vec::new();
        let mut rec: Vec<String> = Vec::new();
        Solution::backtrace(0, &s_chars, &mut rt, &mut rec);

        rt
    }
}

#[test]
fn test_131() {
    println!("result={:?}", Solution::partition("aab".to_string()));
}

#[test]
fn tmp_test() {
    fn main() {
        let vec0 = Vec::new();
        let mut vec1 = fill_vec(vec0);
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
        vec1.push(88);
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }
    fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
        vec.push(22);
        vec.push(44);
        vec.push(66);
        vec
    }
    main();
}
