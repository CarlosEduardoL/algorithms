package carlos.algorithms.sort


import static carlos.algorithms.sort.InsertionSort.insertionSort

class InsertionSortTest extends AbstractSortTest {

    @Override
    void sort(List elements) {
        insertionSort(elements)
    }
}
