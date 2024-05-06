class Solution:
    def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
        n = len(nums)
        result = 0
        cur_prod = 1
        left = 0
        for right in range(n):
            cur_prod *= nums[right]
            while cur_prod >= k and left <= right:
                cur_prod /= nums[left]
                left += 1
            if left <= right:
                result += right - left + 1
        return result
