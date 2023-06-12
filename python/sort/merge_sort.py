def merge_sort(elements: list) -> None:
    """
        Sorts the input list in ascending order using the Merge Sort algorithm.

        Here are some steps to understand how merge sort works:
        1. Divide the list into two halves.
        2. Recursively sort each half.
        3. Merge two halves.

        One of the main advantages of merge sort is that it has a time complexity of O(n log n),
        which means it can sort large lists relatively quickly. It is also a stable sort,
        which means that the order of elements with equal values is preserved during the sort.

        :param elements: The list of elements to sort.
        :return: None
        """
    if len(elements) <= 1:
        return

    mid = len(elements) // 2
    left = elements[:mid]
    right = elements[mid:]
    merge_sort(left)
    merge_sort(right)

    # Merge the two sorted halves into a single sorted list
    merged = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i])
            i += 1
        else:
            merged.append(right[j])
            j += 1
    merged.extend(left[i:])
    merged.extend(right[j:])
    elements[:] = merged
