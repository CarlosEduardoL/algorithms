package sort

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class BubbleSortTest: SortTest() {
    override fun sort(elements: MutableList<Int>) = bubbleSort(elements)

}
