package sort

import java.util.Collections

/**
 * Sorts the input slice in ascending order using the Bubble Sort algorithm.
 *
 * Bubble Sort makes n-1 passes, where in each pass, it swaps all the
 * elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
 * It has a worst-case time complexity of O(n^2).
 *
 * @param elements The list of elements to sort
 */
fun <T: Comparable<T>> bubbleSort(elements: MutableList<T>) {
    // If the list has 1 or 0 elements, it is already sorted
    if (elements.size <= 1) return

    for (k in elements.lastIndex downTo 0) {
        var swapped = false
        for (index in 0 until k) {
            // If adjacent elements are in the wrong order, swap them
            if (elements[index] > elements[index+1]) {
                Collections.swap(elements, index, index+1)
                swapped = true
            }
        }
        // If no swaps were made in the inner loop, the list is sorted
        if (!swapped) return
    }
}