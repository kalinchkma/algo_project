import dobuly_linked_list as DList

"""
Example of useage
"""
# Create a doubly linked list
dll = DList.DoublyLinkedList()

# Read employee data from file
num_employees = int(input("Enter number of employees (5-15): "))
dll.read_from_file("Employees.txt", num_employees)

print("\nOriginal List:")
dll.display()

# Sort by unique ID using merge sort
dll.sort(key="unique_id", algorithm="merge")
print("\nSorted by Unique ID (Merge Sort):")
dll.display()

# Sort by total tasks using merge sort
# dll.sort(key="total_tasks", algorithm="merge")
# print("\nSorted by Total Tasks (Merge Sort):")
# dll.display()

# Sort by average tasks using merge sort
# dll.sort(key="average_tasks", algorithm="merge")
# print("\nSorted by Average Tasks (Merge Sort):")
# dll.display()

# Sort by unique ID using quick sort
# dll.sort(key="unique_id", algorithm="quick")
# print("\nSorted by Unique ID (Quick Sort):")
# dll.display()

# Sort by total tasks using quick sort
# dll.sort(key="total_tasks", algorithm="quick")
# print("\nSorted by Total Tasks (Quick Sort):")
# dll.display()

# Sort by average tasks using quick sort
# dll.sort(key="average_tasks", algorithm="quick")
# print("\nSorted by Average Tasks (Quick Sort):")
# dll.display()

# Calculate median and mode
dll.calculate_median_mode()

# Calculate consistency score
dll.calculate_consistency_score()

# Determine performance trend
dll.determine_performance_trend()

# Generate summary report
dll.generate_summary_report("Employees-Summary.txt")
