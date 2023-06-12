from typing import List


def quicksort(elements: List):
    result = quick_sort(elements)
    elements.clear()
    elements.extend(result)


def quick_sort(elements: List) -> List:
    """
    Sorts a list of elements using the QuickSort algorithm.

    QuickSort is a divide-and-conquer sorting algorithm that works by
    partitioning the input list into two sublists around a pivot element,
    such that all elements in the left sublist are less than the pivot
    element and all elements in the right sublist are greater than or equal
    to the pivot element. The two sublists are then sorted recursively.

    Best Case:
        The best case for QuickSort occurs when the pivot element is always chosen
        as the median element of the list. In this case, the list is always
        partitioned into two sublists of equal size, and the depth of recursion
        is `O(log n)`. The time complexity of QuickSort in the best case is `O(n log n)`.

    Worst Case:
        The worst case for QuickSort occurs when the pivot element is always chosen
        as the smallest or largest element of the list. In this case, one of the
        sublists is always empty, and the depth of recursion is `O(n)`. The time
        complexity of QuickSort in the worst case is `O(n^2)`.

    Arguments:
        elements (list): The list of elements to sort.

    Example:
        >>> arr = [5, 2, 1, 4, 3]
        >>> quicksort(arr)
        >>> print(arr)
        [1, 2, 3, 4, 5]
    """
    if not isinstance(elements, list):
        raise TypeError("elements must be a list")

    # Base case: if the list has length less than or equal to 1,
    # it is already sorted.
    if len(elements) <= 1:
        return elements.copy()

    # Partition the list around a pivot element.
    pivot = elements[len(elements) // 2]

    left = [x for x in elements if x < pivot]
    middle = [x for x in elements if x == pivot]
    right = [x for x in elements if x > pivot]
    return quick_sort(left) + middle + quick_sort(right)
