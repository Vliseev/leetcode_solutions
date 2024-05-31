class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        for i in range(len(nums)):
            while 1 <= nums[i] <= len(nums) and nums[nums[i] - 1] != nums[i]:
                num_id = nums[i] - 1
                nums[num_id], nums[i] = nums[i] , nums[num_id]
        for i in range(len(nums)):
            if nums[i] != i + 1 :
                return i + 1
        return len(nums) + 1
