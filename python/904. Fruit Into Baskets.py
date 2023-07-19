from collections import defaultdict

class Solution:
    def totalFruit(self, fruits: List[int]) -> int:
        cntr = defaultdict(int)
        n = len(fruits)
        i, j = 0, 0
        result = 1
        while i < n:
            cntr[fruits[i]] += 1
            while len(cntr) > 2:
                cntr[fruits[j]] -= 1
                if cntr[fruits[j]] == 0:
                    cntr.pop(fruits[j])
                j += 1
            result = max(result, i - j + 1)
            i += 1
        return result
