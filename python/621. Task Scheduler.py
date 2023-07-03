from collections import Counter, deque
from heapq import heappush, heappop, heapify

class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        result = 0
        queue = deque()

        cntr = Counter(tasks)
        hp = [-el for el in cntr.values()]
        heapify(hp)

        step = 0
        while hp or queue:
            while queue and queue[0][1] < step:
                val, _ = queue.popleft()
                heappush(hp ,val)
            if hp:
                task = -1 * heappop(hp)
                task -= 1
                if task:
                    queue.append((-1 * task, n+step))
            step += 1
        return step
