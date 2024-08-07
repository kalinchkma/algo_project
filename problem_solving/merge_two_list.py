"""
Merge two array
"""


def mergeTwoList(list1, list2):
    merged = []
    i, j = 0, 0

    # iterate first merge list
    while i < len(list1) and j < len(list2):
        if list1[i] < list2[j]:
            merged.append(list1[i])
            i += 1
        else:
            merged.append(list2[j])
            j += 1

    # iterate remaining item
    while i < len(list1):
        merged.append(list1[i])
        i += 1

    # iterate remaining item
    while j < len(list2):
        merged.append(list2[j])
        j += 1

    return merged


if __name__ == "__main__":
    list1 = [1, 1, 2, 4]
    list2 = [1, 3, 3, 4]
    print("list1", list1)
    print("list2", list2)
    print("Output", mergeTwoList(list1, list2))
