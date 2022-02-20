/**
 * [0532] K-diff Pairs in an Array
 *
 * Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.
 * A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:
 *
 * 	0 <= i, j < nums.length
 * 	i != j
 * 	nums[i] - nums[j] == k
 *
 * Notice that |val| denotes the absolute value of val.
 *  
 * Example 1:
 *
 * Input: nums = [3,1,4,1,5], k = 2
 * Output: 2
 * Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
 * Although we have two 1s in the input, we should only return the number of unique pairs.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5], k = 1
 * Output: 4
 * Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).
 *
 * Example 3:
 *
 * Input: nums = [1,3,1,5,4], k = 0
 * Output: 1
 * Explanation: There is one 0-diff pair in the array, (1, 1).
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^7 <= nums[i] <= 10^7
 * 	0 <= k <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-diff-pairs-in-an-array/
// discuss: https://leetcode.com/problems/k-diff-pairs-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = std::collections::HashMap::new();
        nums.iter().for_each(|&x| *hm.entry(x).or_insert(0) += 1);

        let mut count = 0;
        hm.iter().for_each(|(&x, &n)| {
            if hm.get(&(x + k)).is_some() {
                count += if k > 0 || k == 0 && n > 1 {
                    1
                } else {
                    0
                };
            }
        });

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0532_example_1() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        let result = 2;

        assert_eq!(Solution::find_pairs(nums, k), result);
    }

    #[test]
    fn test_0532_example_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 1;
        let result = 4;

        assert_eq!(Solution::find_pairs(nums, k), result);
    }

    #[test]
    fn test_0532_example_3() {
        let nums = vec![1, 3, 1, 5, 4];
        let k = 0;
        let result = 1;

        assert_eq!(Solution::find_pairs(nums, k), result);
    }
}
