def insertion_sort_in_place(elements: list) -> None:
    """
    Sorts the input list in ascending order using the Insertion Sort algorithm.
    Insertion sort is a simple algorithm that sorts a list of items by repeatedly inserting each item into
    its correct position within the sorted portion of the list. To do this, we compare the current item with
    the items to its left until we find its correct position, then we insert it.

    This process is repeated for each item in the list until they are all in their correct positions.

    Insertion Sort has a worst-case time complexity of O(n^2).

    :param elements: List of elements to sort in place.
    :raises TypeError: If elements is not a list.
    """
    if not isinstance(elements, list):
        raise TypeError("elements should be a list")

    if len(elements) <= 1:
        return

    # Iterate throughout all the list.
    for i in range(1, len(elements)):
        # Index of the element to the left of the current selected element
        previous_index = i - 1
        # Selected element
        current_element = elements[i]
        # Move the selected element left until is in the correct position
        while previous_index >= 0 and elements[previous_index] > current_element:
            elements[previous_index + 1] = elements[previous_index]
            previous_index = previous_index - 1

        # Insert the current element at its correct position
        elements[previous_index + 1] = current_element
