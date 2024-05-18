class Solution:
    def longestOnes(self, nums: List[int], k: int) -> int:
        beg = 0
        num_zeros = 0
        res = 0

        for end in range(len(nums)):
            if nums[end] == 0:
                num_zeros += 1
            while num_zeros > k and beg < end:
                if nums[beg] == 0:
                    num_zeros -= 1
                beg += 1
            if num_zeros <= k:
                res = max(res, end - beg + 1)
        return res
  
