class Solution:
    def firstCompleteIndex(self, arr: List[int], mat: List[List[int]]) -> int:
        pos = {}
        n = len(mat)
        m = len(mat[0])
        for i in range(n):
            for j in range(m):
                pos[mat[i][j]] = (i, j)
        row_cnt = [0] * n
        col_cnt = [0] * m
        for i in range(len(arr)):
            x, y = pos[arr[i]]
            row_cnt[x] += 1
            col_cnt[y] += 1
            if row_cnt[x] == m or col_cnt[y] == n:
                return i
        return -1
