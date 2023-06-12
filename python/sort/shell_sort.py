from typing import List


def shell_sort(elements: List):
    """
    Sorts the input list in ascending order using the Shell Sort algorithm.

    The Shell Sort algorithm is an optimization of the Insertion Sort algorithm. It works by
    repeatedly dividing the list into smaller sublists and sorting them using Insertion Sort.

    Steps:
        1. Choose a gap sequence: a series of indices that will divide the list into sublists.
        2. Starting with the largest gap, divide the list into sublists of that size.
        3. Sort each sublist using Insertion Sort.
        4. Repeat with the next smallest gap until the gap is 1.

    - Best Case is O(n)
    - Worst Case is O(n^2)
    - Average Case is generally believed to be between O(n^1.5) and O(n^1.25)

    Arguments:
        elements (list): The list of elements to sort.

    Example:
        >>> v = [3, 2, 1]
        >>> shell_sort(v)
        >>> print(v)
        [1, 2, 3]
    """
    if not isinstance(elements, list):
        raise TypeError("elements must be a list")

    length = len(elements)
    if length <= 1:
        return
    gap = length // 2
    while gap > 0:
        # Perform Insertion Sort on each gap-sized sublist
        for i in range(gap, length):
            j = i
            # Swap elements until they are in order, with a gap-sized step
            while j >= gap and elements[j - gap] > elements[j]:
                elements[j], elements[j - gap] = elements[j - gap], elements[j]
                j -= gap
        # Swap elements until they are in order, with a gap-sized step
        gap //= 2
