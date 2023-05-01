package sort

class MergeSortTest: SortTest() {
    override fun sort(elements: MutableList<Int>) = mergeSort(elements)
}
