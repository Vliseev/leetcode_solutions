class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        dp = {} # (index, buy_or_sell)
        def rec(i, buy):
            if i >= len(prices):
                return 0
            if (i, buy) in dp:
                return dp[(i, buy)]
            cd = rec(i + 1, buy)
            if buy:
                res = max(cd, rec(i + 1, False) - prices[i])
                dp[(i, buy)] = res
            else:
                """
                # i+1 buy - restriction
                i+1 sell - cd
                i+2 buy
                """
                res = max(cd, rec(i + 2, True) + prices[i]) 
                dp[(i, buy)] = res
            return dp[(i, buy)]
        return rec(0, True)
