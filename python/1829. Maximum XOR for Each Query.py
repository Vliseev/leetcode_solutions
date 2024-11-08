class Solution:
    def getMaximumXor(self, nums: List[int], maximumBit: int) -> List[int]:
        cum_xor = 0
        for n in nums:
            cum_xor ^= n
        max_num = (1 << maximumBit) - 1
        result = []
        n = len(nums)
        for i in range(n):
            k = max_num ^ cum_xor
            result.append(k)
            cum_xor ^= nums[-1]
            nums.pop()
        return result
