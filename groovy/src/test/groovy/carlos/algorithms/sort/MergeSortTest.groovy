package carlos.algorithms.sort

import static carlos.algorithms.sort.MergeSort.mergeSort

class MergeSortTest extends AbstractSortTest {

    @Override
    void sort(List elements) {
        mergeSort(elements)
    }
}
