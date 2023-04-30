package carlos.algorithms.sort

import org.junit.jupiter.api.Test

import static carlos.algorithms.sort.BubbleSort.bubbleSort
import static org.junit.jupiter.api.Assertions.assertEquals

class BubbleSortTest {
    @Test
    void testSortEmptyList() {
        // Test sorting an empty list
        def elements = []
        bubbleSort(elements)
        assertEquals([], elements)
    }

    @Test
    void testSortOneElementList() {
        // Test sorting a list with one element
        def elements = [5]
        bubbleSort(elements)
        assertEquals([5], elements)
    }

    @Test
    void testSortSortedList() {
        // Test sorting a list that is already sorted
        def elements = [1, 2, 3, 4, 5]
        bubbleSort(elements)
        assertEquals([1, 2, 3, 4, 5], elements)
    }

    @Test
    void testSortReverseSortedList() {
        // Test sorting a list that is sorted in reverse order
        def elements = [5, 4, 3, 2, 1]
        bubbleSort(elements)
        assertEquals([1, 2, 3, 4, 5], elements)
    }

    @Test
    void testSortUnsortedList() {
        // Test sorting a list that is unsorted
        def elements = [3, 5, 1, 4, 2]
        bubbleSort(elements)
        assertEquals([1, 2, 3, 4, 5], elements)
    }

    @Test
    void testSortListWithDuplicates() {
        // Test sorting a list with duplicate elements
        def elements = [3, 5, 1, 4, 2, 3]
        bubbleSort(elements)
        assertEquals([1, 2, 3, 3, 4, 5], elements)
    }

}
