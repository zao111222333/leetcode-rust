/**
 * [139] Word Break
 *
 * Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.
 *
 * Note:
 *
 *
 * 	The same word in the dictionary may be reused multiple times in the segmentation.
 * 	You may assume the dictionary does not contain duplicate words.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "leetcode", wordDict = ["leet", "code"]
 * Output: true
 * Explanation: Return true because "leetcode" can be segmented as "leet code".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "applepenapple", wordDict = ["apple", "pen"]
 * Output: true
 * Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
 *              Note that you are allowed to reuse a dictionary word.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
 * Output: false
 *
 *
 */
pub struct Solution {}

/*
记 f[n] 表示从 0 开始长度为 n 的 substring 是否可以被组成，那么：

f[n] = f[k] && (s[k..n] in dict)
f[0] = true

DP 向上递推即可

BFS 也是可以的
*/

// problem: https://leetcode.com/problems/word-break/
// discuss: https://leetcode.com/problems/word-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: std::collections::HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp: Vec<bool> = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in (0..=i - 1).rev() {
                if dp[j] && dict.contains(&s[j..=i - 1]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0139_example_1() {
        let s = "leetcode".to_string();
        let word_dict = vec_string!["leet", "code"];
        let result = true;

        assert_eq!(Solution::word_break(s, word_dict), result);
    }

    #[test]
    fn test_0139_example_2() {
        let s = "catsandog".to_string();
        let word_dict = vec_string!["cats", "dog", "sand", "and", "cat"];
        let result = false;

        assert_eq!(Solution::word_break(s, word_dict), result);
    }
}
