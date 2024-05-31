class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result = []

        cols = set()
        main_diag = set()
        side_diag = set()

        board = [['.' for _ in range(n)] for _ in range(n)]

        def backtrack(row):
            if row == n:
                result.append(["".join(row) for row in board])
                return
            for col in range(n):
                if col in cols or row + col in main_diag or row - col in side_diag:
                    continue
                cols.add(col)
                main_diag.add(row + col)
                side_diag.add(row - col)
                board[row][col] = "Q"
                backtrack(row+1)
                board[row][col] = "."
                cols.remove(col)
                main_diag.remove(row + col)
                side_diag.remove(row - col)
        backtrack(0)
        return result
