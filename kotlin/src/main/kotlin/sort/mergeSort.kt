package sort

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
 * @param T The type of elements in the list
 * @param elements The list of elements to sort
 */
fun <T : Comparable<T>> mergeSort(elements: MutableList<T>) {
    if (elements.size <= 1) return
    val mid = elements.size / 2
    // Split elements in two half
    val left = elements.subList(0, mid).toMutableList()
    val right = elements.subList(mid, elements.size).toMutableList()
    mergeSort(left)
    mergeSort(right)

    // Merge the two sorted halves into a single sorted list
    val merged = mutableListOf<T>()
    var i = 0
    var j = 0
    while (i < left.size && j < right.size) {
        if (left[i] <= right[j]) {
            merged.add(left[i])
            i++
        } else {
            merged.add(right[j])
            j++
        }
    }
    merged.addAll(left.subList(i, left.size))
    merged.addAll(right.subList(j, right.size))
    for (k in merged.indices) {
        elements[k] = merged[k]
    }
}