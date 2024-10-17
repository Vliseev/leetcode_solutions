class Solution:
    def maximumSwap(self, num: int) -> int:
        nums = list(str(num))
        last_id = {int(d): i for i, d in enumerate(nums)}
        for i, digit in enumerate(nums):
            for d in range(9, int(digit), -1):
                if last_id.get(d, -1) > i:
                    nums[i], nums[last_id[d]] = nums[last_id[d]], nums[i]
                    return int("".join(nums))
        return int("".join(nums))
