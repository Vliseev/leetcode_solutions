impl Solution {
    /// # Longest Fibonacci Subsequence
    /// 
    /// This function finds the length of the longest subsequence in `arr` that forms a
    /// Fibonacci sequence. A Fibonacci subsequence is one where each element is the sum
    /// of the two preceding elements.
    /// 
    /// ## Algorithm
    /// 
    /// The solution uses dynamic programming with a 2D table:
    /// - `dp[i][j]` represents the length of the longest Fibonacci subsequence ending with 
    ///   elements at indices i and j (where arr[i] < arr[j])
    /// - All entries are initialized to 2 (minimum length of a potential Fibonacci subsequence)
    /// 
    /// ## Steps:
    /// 
    /// 1. Create a hashmap mapping each value to its index for O(1) lookups
    /// 2. For each pair (i,j) where i < j:
    ///    - Calculate the preceding element needed (prev = arr[j] - arr[i])
    ///    - If prev exists in the array at index k where k < i:
    ///      - Update dp[i][j] = dp[k][i] + 1
    ///      - Update max_length if necessary
    /// 3. Return max_length if >= 3, otherwise return 0
    /// 
    /// ## Time Complexity: O(n²)
    /// ## Space Complexity: O(n²)
    /// 
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![2; n]; n];
        
        // Create a hashmap for O(1) value lookups
        let index: std::collections::HashMap<i32, usize> = arr
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect();
        
        // Track the maximum length found
        let mut max_length = 0;
        
        // Iterate through all possible pairs
        for j in 0..n {
            for i in 0..j {
                // Look for the previous value in the potential sequence
                let prev = arr[j] - arr[i];
                
                // Check if we can find a valid previous element
                if prev < arr[i] {
                    if let Some(&k) = index.get(&prev) {
                        dp[i][j] = dp[k][i] + 1;
                        max_length = max_length.max(dp[i][j]);
                    }
                }
            }
        }
        
        // Return 0 if no valid sequence of length >= 3 was found
        if max_length < 3 { 0 } else { max_length }
    }
}
