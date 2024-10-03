class Solution:
    def minSubarray(self, nums: List[int], p: int) -> int:
        n = len(nums)
        sum_nums = sum(nums)

        target = sum_nums % p
        if target == 0:
            return 0

        cur_sum = 0
        index_map = {0: -1} # for a subarray with a beginning at the beginning
        result = n

        for i in range(n):
            cur_sum = (cur_sum + nums[i]) % p
            need_val = (cur_sum - target) % p

            if need_val in index_map:
                result = min(result, i - index_map[need_val])
            index_map[cur_sum] = i
        return result if result < n else -1
