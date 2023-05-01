package sort

import "golang.org/x/exp/constraints"

// InsertionSortInPlace sorts the input slice in ascending order using the Insertion Sort algorithm.
//
// Insertion sort is a simple algorithm that sorts a list of items by repeatedly inserting each item into its correct
// position within the sorted portion of the list. To do this, we compare the current item with the items to its left
// until we find its correct position, then we insert it.
//
// This process is repeated for each item in the list until they are all in their correct positions.
//
// Insertion Sort has a worst-case time complexity of O(n^2).
func InsertionSort[T constraints.Ordered](elements []T) {
	if len(elements) <= 1 {
		return
	}

	// Iterate throughout all the slice
	for i := 1; i < len(elements); i++ {
		// Index of the element to the left of the current selected element
		previousIndex := i - 1
		// Selected element
		currentElement := elements[i]
		// Move the selected element left until is in the correct position
		for previousIndex >= 0 && elements[previousIndex] > currentElement {
			elements[previousIndex+1] = elements[previousIndex]
			previousIndex--
		}
		// Insert the current element at its correct position
		elements[previousIndex+1] = currentElement
	}
}
