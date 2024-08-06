class ListNode(object):
    def __init__(self, val=0, next=None) -> None:
        self.val = val
        self.next = next

    def __str__(self) -> str:
        return f"{self.val} {self.next}"


def mergeTwoList(list1: ListNode, list2: ListNode) -> ListNode:
    dummy = ListNode()

    merge = dummy

    while list1 and list2:
        if list1.val < list2.val:
            merge.next = list1
            list1 = list1.next
        else:
            merge.next = list2
            list2 = list2.next
        merge = merge.next

    if list1:
        merge.next = list1
    elif list2:
        merge.next = list2

    return dummy.next


# Helper function to create a linked list from a list
def create_linked_list(values):
    if not values:
        return None
    head = ListNode(values[0])
    current = head
    for val in values[1:]:
        current.next = ListNode(val)
        current = current.next
    return head


# Helper function to print a linked list
def print_linked_list(head):
    current = head
    while current:
        print(current.val, end=" -> ")
        current = current.next
    print("None")


# Example lists
list1 = create_linked_list([1, 2, 4])
list2 = create_linked_list([1, 3, 4])

merged_list = mergeTwoList(list1, list2)

# Print merged list
print_linked_list(merged_list)
