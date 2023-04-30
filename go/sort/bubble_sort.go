package sort

import "golang.org/x/exp/constraints"

// bubbleSort sorts the input slice in ascending order using the Bubble Sort algorithm.
//
// Bubble Sort makes n-1 passes, where in each pass, it swaps all the
// elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
// It has a worst-case time complexity of O(n^2).
//
// elements - The slice of elements to sort.
func bubbleSort[T constraints.Ordered](elements []T) {
	if len(elements) <= 1 {
		return
	}

	lastIndex := len(elements) - 1
	for k := lastIndex; k >= 0; k-- {
		swapped := false
		for index := 0; index < k; index++ {
			if elements[index] > elements[index+1] {
				elements[index], elements[index+1] = elements[index+1], elements[index]
				swapped = true
			}
		}
		if !swapped {
			return
		}
	}
}
