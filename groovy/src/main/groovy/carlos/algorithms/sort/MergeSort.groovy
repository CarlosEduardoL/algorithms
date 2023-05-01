package carlos.algorithms.sort

/**
 * Sorts the input list in ascending order using the Merge Sort algorithm.
 *
 * Here are some steps to understand how merge sort works:
 * 1. Divide the list into two halves.
 * 2. Recursively sort each half.
 * 3. Merge two halves.
 *
 * One of the main advantages of merge sort is that it has a time complexity of O(n log n),
 * which means it can sort large lists relatively quickly. It is also a stable sort,
 * which means that the order of elements with equal values is preserved during the sort.
 *
 * @param elements The list of elements to sort.
 *
 * @example
 * def list = [3, 2, 1]
 * mergeSort(list)
 * assert list == [1, 2, 3]
 */
static <T extends Comparable> void mergeSort(List<T> elements) {
    if (elements.size() <= 1) {
        return
    }

    def mid = elements.size() / 2
    // Split elements in two halves
    def left = elements[0..mid-1]
    def right = elements[mid..-1]
    mergeSort(left)
    mergeSort(right)

    // Merge the two sorted halves into a single sorted list
    def merged = []
    int i = 0, j = 0
    while (i < left.size() && j < right.size()) {
        if (left[i] <= right[j]) {
            merged << left[i]
            i++
        } else {
            merged << right[j]
            j++
        }
    }
    merged += left[i..<left.size()]
    merged += right[j..<right.size()]
    elements[0..<elements.size()] = merged
}
