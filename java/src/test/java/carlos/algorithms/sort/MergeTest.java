package carlos.algorithms.sort;

import java.util.List;

class MergeTest extends SortTests {

    @Override
    void sort(List<Integer> elements) {
        Merge.sort(elements);
    }
}