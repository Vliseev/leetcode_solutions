from typing import List

class Solution:
    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        n, m = len(grid1), len(grid1[0])

        def dfs(i, j):
            if i < 0 or i >= n or j >= m or j < 0 or grid2[i][j] == 0:
                return True

            grid2[i][j] = 0

            is_sub = grid1[i][j] == 1

            for di, dj in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                is_sub &= dfs(i + di, j + dj)
            return is_sub

        num_sub = 0

        for i in range(n):
            for j in range(m):
                if grid2[i][j] == 1:
                    if dfs(i, j):
                        num_sub += 1
        return num_sub
