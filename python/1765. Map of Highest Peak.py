from collections import deque
from typing import List

class Solution:
    def highestPeak(self, isWater: List[List[int]]) -> List[List[int]]:
        n, m = len(isWater), len(isWater[0])

        height = [[-1] * m for _ in range(n)]
        queue = deque()

        for i in range(n):
            for j in range(m):
                if isWater[i][j] == 1:
                    height[i][j] = 0
                    queue.append((i, j))
        direction = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        while queue:
            i, j = queue.popleft()
            h = height[i][j]
            for dx, dy in direction:
                if 0 <= i + dx < n and 0 <= j + dy < m and height[i + dx][j + dy] == -1:
                    height[i + dx][j + dy] = h + 1
                    queue.append((i + dx, j + dy))
        return height
                
