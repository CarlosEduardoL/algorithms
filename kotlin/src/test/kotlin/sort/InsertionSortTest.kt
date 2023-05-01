package sort

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class InsertionSortTest {
    @Test
    fun testSortEmptyList() {
        // Test sorting an empty list
        val elements = mutableListOf<Int>()
        insertionSort(elements)
        assertEquals(emptyList<Int>(), elements)
    }

    @Test
    fun testSortOneElementList() {
        // Test sorting a list with one element
        val elements = mutableListOf(5)
        insertionSort(elements)
        assertEquals(listOf(5), elements)
    }

    @Test
    fun testSortSortedList() {
        // Test sorting a list that is already sorted
        val elements = mutableListOf(1, 2, 3, 4, 5)
        insertionSort(elements)
        assertEquals(listOf(1, 2, 3, 4, 5), elements)
    }

    @Test
    fun testSortReverseSortedList() {
        // Test sorting a list that is sorted in reverse order
        val elements = mutableListOf(5, 4, 3, 2, 1)
        insertionSort(elements)
        assertEquals(listOf(1, 2, 3, 4, 5), elements)
    }

    @Test
    fun testSortUnsortedList() {
        // Test sorting a list that is unsorted
        val elements = mutableListOf(3, 5, 1, 4, 2)
        insertionSort(elements)
        assertEquals(listOf(1, 2, 3, 4, 5), elements)
    }

    @Test
    fun testSortListWithDuplicates() {
        // Test sorting a list with duplicate elements
        val elements = mutableListOf(3, 5, 1, 4, 2, 3)
        insertionSort(elements)
        assertEquals(listOf(1, 2, 3, 3, 4, 5), elements)
    }
}
