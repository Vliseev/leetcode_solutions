class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        beg = 0
        result, num_zeros = 0, 0

        for end in range(len(nums)):
            if nums[end] == 0:
                num_zeros += 1
            while num_zeros > 1:
                if nums[beg] == 0:
                    num_zeros -= 1
                beg += 1
            result = max(result, end - beg + 1)
        return result - 1
