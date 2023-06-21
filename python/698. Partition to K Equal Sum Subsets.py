class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        sum_arr = sum(nums)
        if sum_arr % k != 0:
            return False
        target = sum_arr // k
        nums.sort(reverse=True)

        dp = {}
        def rec(mask, k, cur_sum):
            if k == 0:
                return True
            if cur_sum == target:
                return rec(mask, k-1, 0)
            for j in range(i, len(nums)):
                if ((mask >> j) & 1) or cur_sum + nums[j] > target:
                    continue
                used[j] = True
                if rec(j + 1, k, cur_sum + nums[j]):
                    return True
                used[j] = False
            return False
        return rec(0, k, 0)
