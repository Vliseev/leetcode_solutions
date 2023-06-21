class Solution:
    def findNumberOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp: dict[int, Tuple[int, int]] = {n-1: (1, 1)}
        
        def rec(i):
            if i in dp:
                return dp[i]
            cur_lid = 1
            cur_cnt = 1
            for j in range(i+1, n):
                if nums[i] < nums[j]:
                    lid, cnt = rec(j)
                    if lid + 1 > cur_lid:
                        cur_lid = lid + 1
                        cur_cnt = cnt
                    elif lid + 1 == cur_lid:
                        cur_cnt += cnt
            dp[i] = (cur_lid, cur_cnt)
            return dp[i]
        for i in range(n):
            rec(i)
        lid = max(len_lid for len_lid, _ in dp.values())
        res = 0
        for len_lid, cnt in dp.values():
            if len_lid == lid:
                res += cnt
        return res
