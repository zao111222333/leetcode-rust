/**
 * [0125] Valid Palindrome
 *
 * A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *  
 * Example 1:
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 * Example 2:
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 * Example 3:
 *
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric characters.
 * Since an empty string reads the same forward and backward, it is a palindrome.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2 * 10^5
 * 	s consists only of printable ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut pos_l = 0;
        let mut pos_r = s.len() - 1;
        let bytes = s.as_bytes();
        while pos_l < pos_r {
            match (
                bytes[pos_l].is_ascii_alphanumeric(),
                bytes[pos_r].is_ascii_alphanumeric(),
            ) {
                (true, true) => {}
                (true, false) => {
                    pos_r -= 1;
                    continue;
                }
                (false, true) => {
                    pos_l += 1;
                    continue;
                }
                (false, false) => {
                    pos_l += 1;
                    pos_r -= 1;
                    continue;
                }
            }
            if bytes[pos_l].to_ascii_lowercase() != bytes[pos_r].to_ascii_lowercase() {
                return false;
            }
            pos_l += 1;
            pos_r -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0125_example_1() {
        assert_eq!(
            true,
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
        );
        assert_eq!(true, Solution::is_palindrome(" ".to_string()));
        assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
    }
}
