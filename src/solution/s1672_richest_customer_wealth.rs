/**
 * [1672] Richest Customer Wealth
 *
 * You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the i​​​​​^​​​​​​th​​​​ customer has in the j​​​​​^​​​​​​th​​​​ bank. Return the wealth that the richest customer has.
 * A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.
 *  
 * Example 1:
 *
 * Input: accounts = [[1,2,3],[3,2,1]]
 * Output: 6
 * Explanation:
 * 1st customer has wealth = 1 + 2 + 3 = 6
 * 2nd customer has wealth = 3 + 2 + 1 = 6
 * Both customers are considered the richest with a wealth of 6 each, so return 6.
 *
 * Example 2:
 *
 * Input: accounts = [[1,5],[7,3],[3,5]]
 * Output: 10
 * Explanation:
 * 1st customer has wealth = 6
 * 2nd customer has wealth = 10
 * 3rd customer has wealth = 8
 * The 2nd customer is the richest with a wealth of 10.
 * Example 3:
 *
 * Input: accounts = [[2,8,7],[7,1,3],[1,9,5]]
 * Output: 17
 *
 *  
 * Constraints:
 *
 * 	m == accounts.length
 * 	n == accounts[i].length
 * 	1 <= m, n <= 50
 * 	1 <= accounts[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/richest-customer-wealth/
// discuss: https://leetcode.com/problems/richest-customer-wealth/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|ac| ac.iter().sum()).max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1672_example_1() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];

        let result = 6;

        assert_eq!(Solution::maximum_wealth(accounts), result);
    }

    #[test]
    fn test_1672_example_2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];

        let result = 10;

        assert_eq!(Solution::maximum_wealth(accounts), result);
    }

    #[test]
    fn test_1672_example_3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];

        let result = 17;

        assert_eq!(Solution::maximum_wealth(accounts), result);
    }
}
