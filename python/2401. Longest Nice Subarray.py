from typing import List

class Solution:
    def longestNiceSubarray(self, nums: List[int]) -> int:
        acc = 0

        result = 0
        left = 0
        n = len(nums)

        for right in range(n):
            num = nums[right]
            while num & acc:
                acc = acc ^ nums[left]
                left += 1
            acc = acc | num
            result = max(result, right - left + 1)
        return result
