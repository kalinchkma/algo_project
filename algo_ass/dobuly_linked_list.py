import numpy as np


# Node class of DoublyLinkedList
class Node:
    def __init__(self, unique_id, name, tasks):
        self.unique_id = unique_id
        self.name = name
        self.tasks = tasks
        self.next = None
        self.prev = None


# DoublyLinkedList class
class DoublyLinkedList:
    def __init__(self):
        self.head = None

    def append(self, unique_id, name, tasks):
        new_node = Node(unique_id, name, tasks)
        if self.head is None:
            self.head = new_node
            return
        last = self.head
        while last.next:
            last = last.next
        last.next = new_node
        new_node.prev = last

    # Display the doubly linked list
    def display(self):
        nodes = []
        current = self.head
        while current:
            nodes.append((current.unique_id, current.name, current.tasks))
            current = current.next
        for node in nodes:
            print(f"ID: {node[0]}, Name: {node[1]}, Tasks: {node[2]}")

    # Split the doubly linked list into two halves
    def split(self, head):
        fast = slow = head
        while fast.next and fast.next.next:
            slow = slow.next
            fast = fast.next.next
        temp = slow.next
        slow.next = None
        return head, temp

    # Merge two sorted linked lists
    def merge(self, first, second, key):
        if not first:
            return second
        if not second:
            return first

        if key == "unique_id":
            if first.unique_id < second.unique_id:
                first.next = self.merge(first.next, second, key)
                first.next.prev = first
                first.prev = None
                return first
            else:
                second.next = self.merge(first, second.next, key)
                second.next.prev = second
                second.prev = None
                return second
        elif key == "total_tasks":
            if np.sum(first.tasks) < np.sum(second.tasks):
                first.next = self.merge(first.next, second, key)
                first.next.prev = first
                first.prev = None
                return first
            else:
                second.next = self.merge(first, second.next, key)
                second.next.prev = second
                second.prev = None
                return second
        elif key == "average_tasks":
            if np.mean(first.tasks) < np.mean(second.tasks):
                first.next = self.merge(first.next, second, key)
                first.next.prev = first
                first.prev = None
                return first
            else:
                second.next = self.merge(first, second.next, key)
                second.next.prev = second
                second.prev = None
                return second

    # Merge sort
    def merge_sort(self, node, key):
        if not node or not node.next:
            return node
        first, second = self.split(node)
        first = self.merge_sort(first, key)
        second = self.merge_sort(second, key)
        return self.merge(first, second, key)

    # Sort the doubly linked list based on key using merge sort or quick sort
    def sort(self, key="unique_id", algorithm="merge"):
        if algorithm == "merge":
            self.head = self.merge_sort(self.head, key)
        elif algorithm == "quick":
            self.head = self.quick_sort(self.head, key)

    # Partition the doubly linked list and return the pivot
    def partition(self, low, high, key):
        pivot = high
        i = low.prev
        current = low
        while current != high:
            if key == "unique_id" and current.unique_id <= pivot.unique_id:
                i = low if i is None else i.next
                i.unique_id, current.unique_id = current.unique_id, i.unique_id
                i.name, current.name = current.name, i.name
                i.tasks, current.tasks = current.tasks, i.tasks
            elif key == "total_tasks" and np.sum(current.tasks) <= np.sum(pivot.tasks):
                i = low if i is None else i.next
                i.unique_id, current.unique_id = current.unique_id, i.unique_id
                i.name, current.name = current.name, i.name
                i.tasks, current.tasks = current.tasks, i.tasks
            elif key == "average_tasks" and np.mean(current.tasks) <= np.mean(
                pivot.tasks
            ):
                i = low if i is None else i.next
                i.unique_id, current.unique_id = current.unique_id, i.unique_id
                i.name, current.name = current.name, i.name
                i.tasks, current.tasks = current.tasks, i.tasks
            current = current.next
        i = low if i is None else i.next
        i.unique_id, pivot.unique_id = pivot.unique_id, i.unique_id
        i.name, pivot.name = pivot.name, i.name
        i.tasks, pivot.tasks = pivot.tasks, i.tasks
        return i

    # Quick sort helper function
    def _quick_sort(self, low, high, key):
        if high and low != high and low != high.next:
            p = self.partition(low, high, key)
            self._quick_sort(low, p.prev, key)
            self._quick_sort(p.next, high, key)

    # Quick sort
    def quick_sort(self, head, key):
        tail = head
        while tail and tail.next:
            tail = tail.next
        self._quick_sort(head, tail, key)
        return head

    def linear_search(self, key, value):
        """
        Linear search based on key and value (requires unsorted list)
        """
        current = self.head
        while current:
            if key == "unique_id" and current.unique_id == value:
                return current
            elif key == "name" and current.name == value:
                return current
            current = current.next
        return None

    def binary_search(self, key, value):
        """
        Binary search based on key and value (requires sorted list)
        """
        current = self.head
        nodes = []
        while current:
            nodes.append(current)
            current = current.next

        low, high = 0, len(nodes) - 1
        while low <= high:
            mid = (low + high) // 2
            if key == "unique_id":
                if nodes[mid].unique_id == value:
                    return nodes[mid]
                elif nodes[mid].unique_id < value:
                    low = mid + 1
                else:
                    high = mid - 1
            elif key == "name":
                if nodes[mid].name == value:
                    return nodes[mid]
                elif nodes[mid].name < value:
                    low = mid + 1
                else:
                    high = mid - 1
        return None

    def calculate_median_mode(self):
        """
        Calculate median and mode of tasks for each employee
        """
        current = self.head
        while current:
            median = np.median(current.tasks)
            mode = int(np.bincount(current.tasks).argmax())
            print(f"Employee {current.name}: Median = {median}, Mode = {mode}")
            current = current.next

    def calculate_consistency_score(self):
        """
        Calculate consistency score
        for each employee based on mean and standard deviation of tasks
        """
        current = self.head
        while current:
            mean_tasks = np.mean(current.tasks)
            std_tasks = np.std(current.tasks)
            consistency_score = mean_tasks / std_tasks if std_tasks != 0 else 0
            print(f"Employee {current.name}: Consistency Score = {consistency_score}")
            current = current.next

    def determine_performance_trend(self):
        """
        Determine performance trend
        for each employee based on first half and second half of tasks data
        """
        current = self.head
        while current:
            first_half_avg = np.mean(current.tasks[:6])
            second_half_avg = np.mean(current.tasks[6:])
            if second_half_avg > first_half_avg:
                trend = "Improving"
            elif second_half_avg < first_half_avg:
                trend = "Declining"
            else:
                trend = "Stable"
            print(f"Employee {current.name}: Performance Trend = {trend}")
            current = current.next

    def read_from_file(self, filename, num_employees):
        """
        Read employee data from file and append
        to the doubly linked list (up to num_employees)
        with error handling
        """
        try:
            with open(filename, "r") as file:
                lines = file.readlines()
                if not (5 <= num_employees <= 15):
                    raise ValueError("Number of employees must be between 5 and 15.")
                for line in lines[:num_employees]:
                    parts = line.strip().split(",")
                    if len(parts) != 14:
                        raise ValueError("Incorrect format in data file.")
                    unique_id = int(parts[0])
                    name = parts[1]
                    tasks = np.array([int(x) for x in parts[2:]])
                    self.append(unique_id, name, tasks)
        except FileNotFoundError:
            print(f"File {filename} not found.")
        except ValueError as e:
            print(e)

    def generate_summary_report(self, filename):
        """
        Generate a summary report of employees
        and write to a file with error handling (filename)
        using numpy functions for calculations
        """
        try:
            with open(filename, "w") as file:
                file.write("***Employee Summary Report***\n")
                file.write("=======================\n")
                current = self.head
                while current:
                    median = np.median(current.tasks)
                    mode = int(np.bincount(current.tasks).argmax())
                    mean_tasks = np.mean(current.tasks)
                    std_tasks = np.std(current.tasks)
                    consistency_score = mean_tasks / std_tasks if std_tasks != 0 else 0
                    first_half_avg = np.mean(current.tasks[:6])
                    second_half_avg = np.mean(current.tasks[6:])
                    if second_half_avg > first_half_avg:
                        trend = "Improving"
                    elif second_half_avg < first_half_avg:
                        trend = "Declining"
                    else:
                        trend = "Stable"
                    file.write(f"ID: {current.unique_id}, Name: {current.name}\n")
                    file.write(f"Tasks: {current.tasks}\n")
                    file.write(f"Median: {median}, Mode: {mode}\n")
                    file.write(f"Consistency Score: {consistency_score}\n")
                    file.write(f"Performance Trend: {trend}\n\n")
                    current = current.next

        except Exception as e:
            print(f"Error writing to file {filename}: {e}")
