class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        def is_valid(board, row, col, num):
            for i in range(9):
                if board[row][i] == num:
                    return False
            for i in range(9):
                if board[i][col] == num:
                    return False
            start_row = (row // 3) * 3
            start_col = (col // 3) * 3
            for i in range(3):
                for j in range(3):
                    if board[start_row + i][start_col + j] == num:
                        return False
            return True
        
        def solution(board):
            for i in range(9):
                for j in range(9):
                    if board[i][j] == '.':
                        for ch in '123456789':
                            if is_valid(board, i, j, ch):
                                board[i][j] = ch
                                if solution(board):
                                    return True
                                board[i][j] = '.'
                        return False
            return True
        return solution(board)
