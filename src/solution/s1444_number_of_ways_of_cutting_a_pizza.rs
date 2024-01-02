/**
 * [1444] Number of Ways of Cutting a Pizza
 *
 * Given a rectangular pizza represented as a rows x cols matrix containing the following characters: 'A' (an apple) and '.' (empty cell) and given the integer k. You have to cut the pizza into k pieces using k-1 cuts.
 * For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.
 * Return the number of ways of cutting the pizza such that each piece contains at least one apple. Since the answer can be a huge number, return this modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png" style="width: 500px; height: 378px;" />
 *
 * Input: pizza = ["A..","AAA","..."], k = 3
 * Output: 3
 * Explanation: The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
 *
 * Example 2:
 *
 * Input: pizza = ["A..","AA.","..."], k = 3
 * Output: 1
 *
 * Example 3:
 *
 * Input: pizza = ["A..","A..","..."], k = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= rows, cols <= 50
 * 	rows == pizza.length
 * 	cols == pizza[i].length
 * 	1 <= k <= 10
 * 	pizza consists of characters 'A' and '.' only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/
// discuss: https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/solutions/3112204/just-a-runnable-solution/
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let mut dp = vec![vec![vec![0; pizza[0].len() + 1]; pizza.len() + 1]; k as usize + 1];
        let mut sum = vec![vec![0; pizza[0].len() + 1]; pizza.len() + 1];

        for i in (0..pizza.len()).rev() {
            for j in (0..pizza[0].len()).rev() {
                sum[i][j] = sum[i + 1][j] + sum[i][j + 1] - sum[i + 1][j + 1]
                    + (pizza[i].chars().nth(j).unwrap() == 'A') as i32;
            }
        }

        for (i, item) in sum.iter().enumerate().take(pizza.len()) {
            for (j, &item2) in item.iter().enumerate().take(pizza[0].len()) {
                dp[1][i][j] = (item2 > 0) as i32;
            }
        }

        for l in 2..=k as usize {
            for i in (0..pizza.len()).rev() {
                for j in (0..pizza[0].len()).rev() {
                    for m in i + 1..pizza.len() {
                        if sum[i][j] - sum[m][j] > 0 {
                            dp[l][i][j] += dp[l - 1][m][j];
                            dp[l][i][j] %= 1000000007;
                        }
                    }
                    for n in j + 1..pizza[0].len() {
                        if sum[i][j] - sum[i][n] > 0 {
                            dp[l][i][j] += dp[l - 1][i][n];
                            dp[l][i][j] %= 1000000007;
                        }
                    }
                }
            }
        }

        dp[k as usize][0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1444_example_1() {
        let pizza = vec_string!["A..", "AAA", "..."];
        let k = 3;

        let result = 3;

        assert_eq!(Solution::ways(pizza, k), result);
    }

    #[test]
    fn test_1444_example_2() {
        let pizza = vec_string!["A..", "AA.", "..."];
        let k = 3;

        let result = 1;

        assert_eq!(Solution::ways(pizza, k), result);
    }

    #[test]
    fn test_1444_example_3() {
        let pizza = vec_string!["A..", "A..", "..."];
        let k = 1;

        let result = 1;

        assert_eq!(Solution::ways(pizza, k), result);
    }
}
