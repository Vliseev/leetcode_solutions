class Solution:
    def parseBoolExpr(self, expression: str) -> bool:
        stack = []

        for char in expression:
            if char == ")":
                sub_expr = []
                while stack[-1] != "(":
                    sub_expr.append(stack.pop())
                stack.pop()  # remove (

                operator = stack.pop()
                match operator:
                    case "!":
                        stack.append(not sub_expr[0])
                    case "&":
                        stack.append(all(sub_expr))
                    case "|":
                        stack.append(any(sub_expr))
            elif char != ",":
                match char:
                    case "t":
                        stack.append(True)
                    case "f":
                        stack.append(False)
                    case _:
                        stack.append(char)
        return stack[0]
