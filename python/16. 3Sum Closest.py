from typing import List

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()
        n = len(nums)
        result = nums[0] + nums[1] + nums[2]
        for i in range(n-2):
            j, k = i + 1, n - 1
            while j < k:
                cur_val = nums[i] + nums[j] + nums[k]
                if abs(target - cur_val) < abs (target - result):
                    result = cur_val
                if cur_val == target:
                    return target
                elif cur_val < target:
                    j += 1
                else:
                    k -= 1
        return result
