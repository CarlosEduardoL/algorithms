import unittest
import sorting/bubble_sort

# Define a test suite for the bubble_sort function
suite "Bubble Sort":
  
    # Define a test case for sorting an empty list
    test "Empty list":
        var elements: seq[int] = @[]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[]

    # Define a test case for sorting a list with one element
    test "One element list":
        var elements: seq[int] = @[5]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[5]

    # Define a test case for sorting a list that is already sorted
    test "Sorted list":
        var elements: seq[int] = @[1, 2, 3, 4, 5]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[1, 2, 3, 4, 5]

    # Define a test case for sorting a list that is sorted in reverse order
    test "Reverse sorted list":
        var elements: seq[int] = @[5, 4, 3, 2, 1]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[1, 2, 3, 4, 5]

    # Define a test case for sorting a list that is unsorted
    test "Unsorted list":
        var elements: seq[int] = @[3, 5, 1, 4, 2]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[1, 2, 3, 4, 5]

    # Define a test case for sorting a list with duplicate elements
    test "List with duplicates":
        var elements: seq[int] = @[3, 5, 1, 4, 2, 3]
        bubble_sort.bubble_sort(elements)
        doAssert elements == @[1, 2, 3, 3, 4, 5]