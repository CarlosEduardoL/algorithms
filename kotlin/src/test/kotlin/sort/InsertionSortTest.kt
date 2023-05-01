package sort

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class InsertionSortTest: SortTest() {
    override fun sort(elements: MutableList<Int>) = insertionSort(elements)

}
