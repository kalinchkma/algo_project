def isValid(s):
    """
    :type s: str
    :rtype: bool
    """
    parentheses = {"(": 1, ")": -1, "{": 2, "}": -2, "[": 3, "]": -3}
    pCount = []

    for c in s:
        if c in parentheses:
            pCount.append(parentheses[c])

    if len(pCount) % 2 != 0:
        return False

    for p in range(len(pCount)-1):



if __name__ == "__main__":
    print(isValid("(wew)[we]{weqw}"))
