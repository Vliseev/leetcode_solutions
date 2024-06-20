class Solution:
    def minKBitFlips(self, nums: List[int], k: int) -> int:
        n = len(nums)
        flip_count = 0
        current_flips = 0
        flipped = [0] * n
        
        for i in range(len(nums)):
            if i >= k:
                """
                аффектит ли i - k флип текущий бит
                """
                current_flips ^= flipped[i - k]
            """
            current_flips == 1 nums[i] == 1 - флипаем, потому что накопился суммарный флип
            current_flips == 0 nums[i] == 0 - флипаем, потому что надо
            """
            if current_flips == nums[i]:
                if i + k > n:
                    return -1
                current_flips ^= 1
                flip_count += 1
                flipped[i] = 1

        return flip_count
