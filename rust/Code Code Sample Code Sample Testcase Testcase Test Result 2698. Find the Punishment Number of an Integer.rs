impl Solution {
    /// Returns the "punishment number" for a given integer n, which is the sum of i² for all i from 1 to n
    /// where i² can be partitioned into groups that sum to i.
    /// 
    /// Example: For i=9, 9² = 81
    /// 81 can be partitioned as 8+1=9, so 81 contributes to the punishment number
    pub fn punishment_number(n: i32) -> i32 {
        /// Helper struct to handle the partitioning logic
        struct Partitioner<'a> {
            square_str: &'a str,    // String representation of the square number
            target: i32,            // The original number we're trying to sum to
        }

        impl<'a> Partitioner<'a> {
            /// Recursively checks if the digits in square_str can be partitioned 
            /// into groups that sum to target
            /// 
            /// Args:
            ///     current_sum: Running sum of the partitions processed so far
            ///     index: Current position in the square_str being processed
            ///
            /// Returns:
            ///     bool: true if a valid partition exists, false otherwise
            fn can_partition(&self, current_sum: i32, index: usize) -> bool {
                // If we've processed all digits, check if sum equals target
                if index == self.square_str.len() {
                    return current_sum == self.target;
                }

                let mut num = 0;
                // Try different partition lengths starting from current index
                for i in index..self.square_str.len() {
                    // Build number by adding one digit at a time
                    // Convert char to integer by subtracting ASCII value of '0'
                    num = num * 10 + (self.square_str.as_bytes()[i] - b'0') as i32;
                    
                    // Optimization: Stop if current partition makes sum too large
                    if current_sum + num > self.target {
                        break;
                    }

                    // Try this partition and recursively process remaining digits
                    if self.can_partition(current_sum + num, i + 1) {
                        return true;
                    }
                }
                false
            }
        }

        let mut result = 0;
        // Check each number from 1 to n
        for i in 1..=n {
            let square = i * i;
            let partitioner = Partitioner {
                square_str: &square.to_string(),
                target: i,
            };
            
            // If number can be partitioned correctly, add its square to result
            if partitioner.can_partition(0, 0) {
                result += square;
            }
        }
        result
    }
}
