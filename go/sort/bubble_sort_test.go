package sort

import (
	"reflect"
	"testing"
)

func TestBubbleSort(t *testing.T) {
	// Test sorting an empty list
	elements := []int{}
	bubbleSort(elements)
	if len(elements) != 0 {
		t.Errorf("Expected an empty list, got %v", elements)
	}

	// Test sorting a list with one element
	elements = []int{5}
	bubbleSort(elements)
	if !reflect.DeepEqual(elements, []int{5}) {
		t.Errorf("Expected [5], got %v", elements)
	}

	// Test sorting a list that is already sorted
	elements = []int{1, 2, 3, 4, 5}
	bubbleSort(elements)
	if !reflect.DeepEqual(elements, []int{1, 2, 3, 4, 5}) {
		t.Errorf("Expected [1, 2, 3, 4, 5], got %v", elements)
	}

	// Test sorting a list that is sorted in reverse order
	elements = []int{5, 4, 3, 2, 1}
	bubbleSort(elements)
	if !reflect.DeepEqual(elements, []int{1, 2, 3, 4, 5}) {
		t.Errorf("Expected [1, 2, 3, 4, 5], got %v", elements)
	}

	// Test sorting a list that is unsorted
	elements = []int{3, 5, 1, 4, 2}
	bubbleSort(elements)
	if !reflect.DeepEqual(elements, []int{1, 2, 3, 4, 5}) {
		t.Errorf("Expected [1, 2, 3, 4, 5], got %v", elements)
	}

	// Test sorting a list with duplicate elements
	elements = []int{3, 5, 1, 4, 2, 3}
	bubbleSort(elements)
	if !reflect.DeepEqual(elements, []int{1, 2, 3, 3, 4, 5}) {
		t.Errorf("Expected [1, 2, 3, 3, 4, 5], got %v", elements)
	}
}
