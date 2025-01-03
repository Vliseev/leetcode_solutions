class Solution:
    def waysToSplitArray(self, nums: List[int]) -> int:
        n = len(nums)
        pref_sum = [0] * n
        prev = 0
        for i in range(n):
            pref_sum[i] = prev + nums[i]
            prev = pref_sum[i]
        ans = 0
        tot = pref_sum[-1]
        for i in range(n-1):
            left_sum = pref_sum[i]
            right_sum = tot - left_sum
            if left_sum >= right_sum:
                ans += 1
        return ans
