"""
1. **Count Total Pairs:**  
   \[
   \text{total pairs} = \frac{n \times (n - 1)}{2}
   \]

2. **Count Good Pairs:**
   - Create a dictionary (or hashmap) to count the frequency of each key \( i - \text{nums}[i] \).
   - For each key with frequency \(f\), the number of good pairs is:
     
     \[
     \binom{f}{2} = \frac{f \times (f-1)}{2}
     \]

3. **Calculate Bad Pairs:**  
   \[
   \text{bad pairs} = \text{total pairs} - \text{good pairs}
   \]

**Time Complexity:** \(O(n)\)  
**Space Complexity:** \(O(n)\) (in the worst-case scenario)
"""

from collections import defaultdict
from typing import List

class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        hm = defaultdict(int)
        n = len(nums)

        for i in range(n):
            hm[nums[i] - i] += 1
        
        tot = n * (n - 1) // 2

        for k,v in hm.items():
            tot -= v * (v - 1) // 2
        return tot
