class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        def partition(l, r, k):
            i = l
            for j in range(l, r):
                if nums[j] < nums[r]:
                    nums[j], nums[i] = nums[i], nums[j]
                    i += 1
            nums[i], nums[r] = nums[r], nums[i]
            return i
        l, r = 0, len(nums) - 1
        k = len(nums) - k
        while l < r:
            i = partition(l, r, k)
            if i == k:
                return nums[i]
            elif i < k:
                l = i + 1
            else:
                r = i - 1
        return nums[l]

