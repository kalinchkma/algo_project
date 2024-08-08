"""
--> Valid Palindrome
A phrase is a palindrome if, after converting all uppercase letters
into lowercase letters and removing all non-alphanumeric characters, 
it reads the same forward and backward. 
Alphanumeric characters include letters and numbers.
"""

import math


def palindrome(s: str):
    """
    :type s: str
    :rtype: bool
    """

    if len(s) == 1:
        return True

    chars: str = ""

    # Remove non-alphanumeric characters
    for c in s.lower():
        code = ord(c)
        if (code >= 97 and code <= 122) or (code >= 48 and code <= 57):
            chars += c

    n = len(chars)
    if n == 1:
        return True

    # Calculate middle
    middle = math.floor(n / 2)

    i, j = 0, n - 1

    while i <= middle and j >= middle:
        if chars[i] != chars[j]:
            return False
        i += 1
        j -= 1

    return True


import re


# solve with regex
def isPalindrome(s):
    a = s.lower()
    f = re.sub(r"[^A-Za-z0-9]", "", a)
    b = f[::-1]
    return f == b


if __name__ == "__main__":
    s = "A man, a plan, a canal: Panama"

    # print(palindrome(s))
    # print(palindrome("race a car"))
    # print(palindrome("0P"))
    # print(palindrome("A man, a plan, a anal: Panama"))

    print(isPalindrome(s))
    print(isPalindrome("race a car"))
    print(isPalindrome("0P"))
    print(isPalindrome("A man, a plan, a anal: Panama"))
