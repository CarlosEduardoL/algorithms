package sort

import "golang.org/x/exp/constraints"

// MergeSort sorts the input slice in ascending order using the Merge Sort algorithm.
//
// Here are some steps to understand how merge sort works:
// 1. Divide the array into two halves.
// 2. Recursively sort each half.
// 3. Merge two halves.
//
// One of the main advantages of merge sort is that it has a time complexity of O(n log n),
// which means it can sort large arrays relatively quickly. It is also a stable sort,
// which means that the order of elements with equal values is preserved during the sort.
//
// Arguments:
//
// * `elements` - The slice of elements to sort.
//
// Example:
//
// ```
// v := []int{3, 2, 1}
// MergeSort(v)
// fmt.Println(v) // Output: [1 2 3]
// ```
func MergeSort[T constraints.Ordered](elements []T) {
	if len(elements) <= 1 {
		return
	}
	mid := len(elements) / 2
	// Split elements in two half
	left := elements[:mid]
	right := elements[mid:]
	MergeSort(left)
	MergeSort(right)

	// Merge the two sorted halves into a single sorted slice
	merged := make([]T, 0, len(elements))
	i, j := 0, 0
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			merged = append(merged, left[i])
			i++
		} else {
			merged = append(merged, right[j])
			j++
		}
	}
	merged = append(merged, left[i:]...)
	merged = append(merged, right[j:]...)
	copy(elements, merged)
}
