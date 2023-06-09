package carlos.algorithms.sort

/**
 * Sorts the input list in ascending order using the Insertion Sort algorithm.
 *
 * Insertion sort is a simple algorithm that sorts a list of items by repeatedly inserting each item into
 * its correct position within the sorted portion of the list. To do this, we compare the current item with
 * the items to its left until we find its correct position, then we insert it.
 *
 * This process is repeated for each item in the list until they are all in their correct positions.
 *
 * Insertion Sort has a worst-case time complexity of O(n^2).
 *
 * @param elements List of elements to sort in place.
 */
static <T extends Comparable<T>> void insertionSort(List<T> elements) {
    if (elements.size() <= 1) return
    // Iterate throughout all the list.
    for (i in 1..<elements.size()) {
        // Index of the element to the left of the current selected element
        int previousIndex = i - 1
        // Selected element
        def currentElement = elements[i]
        // Move the selected element left until is in the correct position
        while (previousIndex >= 0 && elements[previousIndex] > currentElement) {
            elements[previousIndex + 1] = elements[previousIndex]
            previousIndex--
        }

        // Insert the current element at its correct position
        elements[previousIndex + 1] = currentElement
    }
}