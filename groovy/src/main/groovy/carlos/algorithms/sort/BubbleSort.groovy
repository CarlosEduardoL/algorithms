package carlos.algorithms.sort

/**
 * Sorts the input list in ascending order using the Bubble Sort algorithm in-place.
 * <p>
 * Bubble Sort makes n-1 passes, where in each pass, it swaps all the
 * elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
 * It has a worst-case time complexity of O(n^2).
 *
 * @param elements List of elements to sort.
 * @param <T>      Any comparable type
 * @throws IllegalArgumentException If the input list is null.
 */
static <T extends Comparable<T>> void bubbleSort(List<T> elements) {

    if (elements == null) throw IllegalArgumentException("elements must be a List")

    if (elements.size() <= 1) {
        return  // Early return if the list only has one or zero elements
    }

    def last_index = elements.size() - 1

    // Iterate over the list n-1 times
    for (def k = last_index; k > 0; k--) {
        def swapped = false
        for (def index = 0; index < k; index++) {
            // If adjacent elements are in the wrong order, swap them
            if (elements[index] > elements[index + 1]) {
                swapped = true
                def temp = elements[index]
                elements[index] = elements[index + 1]
                elements[index + 1] = temp
            }
        }
        if (!swapped) {
            return
        }
    }
}