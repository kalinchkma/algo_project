def isValid(s):
    """
    :type s: str
    :rtype: bool
    """
    stack = []
    parentheses = {")": "(", "}": "{", "]": "["}
    opening = ["(", "{", "["]

    for char in s:
        if char in parentheses:
            top = stack.pop() if stack else "#"
            if parentheses[char] != top:
                return False
        elif char in opening:
            # push the current opening bracket onto the stack
            stack.append(char)
    return not stack


if __name__ == "__main__":
    print(isValid("(wew)[we]{weqw}[]"))
