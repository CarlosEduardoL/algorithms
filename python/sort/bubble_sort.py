def bubble_sort_in_place(elements: list):
    """
    Sorts the input list in ascending order using the Bubble Sort algorithm.
    Bubble Sort makes n-1 passes, where in each pass, it swaps all the
    elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
    It has a worst-case time complexity of O(n^2).

    :param elements: List of elements to test_sort
    :raises TypeError: If `elements` is not a list.
    """
    # Ensure the input parameter is a list
    if not isinstance(elements, list):
        raise TypeError("Input must be a list.")

    if len(elements) <= 1:
        return  # A list with one or zero elements is always sorted.

    last_index = len(elements) - 1

    # Iterate over the list n-1 times
    for k in range(last_index, 0, -1):
        swapped = False
        for index in range(k):
            # If adjacent elements are in the wrong order, swap them
            if elements[index] > elements[index + 1]:
                elements[index], elements[index + 1] = elements[index + 1], elements[index]
                swapped = True
        if not swapped:
            return


def bubble_sort(elements: list) -> list:
    """
    Returns a sorted copy of the input list using the Bubble Sort algorithm.
    Bubble Sort makes n-1 passes, where in each pass, it swaps all the
    elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
    It has a worst-case time complexity of O(n^2).

    :param elements: List of elements to test_sort.
    :return: A sorted copy of the input list.
    """
    # Make a copy of the input list
    elements_copy = elements.copy()

    if len(elements_copy) <= 1:
        return elements_copy  # Early return if the list only has one or zero elements

    # Sort the copy using the bubble test_sort algorithm
    bubble_sort_in_place(elements_copy)

    return elements_copy
