from typing import List
import heapq

class Solution:
    def trapRainWater(self, heightMap: List[List[int]]) -> int:
        n, m = len(heightMap), len(heightMap[0])
        heap = []
        visited = [[False] * m for _ in range(n)]

        for i in range(n):
            for j in range(m):
                if i == 0 or i == n - 1 or j == 0 or j == m - 1:
                    heapq.heappush(heap, (heightMap[i][j], i, j))
                    visited[i][j] = True
        
        result = 0
        directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]

        while heap:
            height, i, j = heapq.heappop(heap)

            for dx, dy in directions:
                x, y = i + dx, j + dy
                if 0 <= x < n and 0 <= y < m and not visited[x][y]:
                    result += max(0, height - heightMap[x][y])
                    heapq.heappush(heap, (max(heightMap[x][y], height), x, y))
                    visited[x][y] = True
        return result
