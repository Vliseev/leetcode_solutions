class Solution:
    def longestDiverseString(self, a: int, b: int, c: int) -> str:
        heap = []
        if a:
            heapq.heappush(heap, (-a, 'a'))
        if b:
            heapq.heappush(heap, (-b, 'b'))
        if c:
            heapq.heappush(heap, (-c, 'c'))

        result = []
        
        while heap:
            num1, ch1 = heapq.heappop(heap)
            num1 = -num1
            if len(result) >= 2 and ch1 == result[-1] and ch1 == result[-2]:
                if heap:
                    num2, ch2 = heapq.heappop(heap)
                else:
                    break
                num2 = -num2
                num2 -= 1
                result.append(ch2)
                if num2:
                    heapq.heappush(heap, (-num2, ch2))
                heapq.heappush(heap, (-num1, ch1))
            else:
                num1 -= 1
                result.append(ch1)
                if num1:
                    heapq.heappush(heap, (-num1, ch1))
        return "".join(result)
