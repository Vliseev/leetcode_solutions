class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()
        n = len(nums)
        res = 0
        for i, x in enumerate(nums):
            l = bisect_left(nums, lower - x, lo = i+1)
            r = bisect_left(nums, upper - x + 1, lo = i+1)
            res += r - l
        return res
