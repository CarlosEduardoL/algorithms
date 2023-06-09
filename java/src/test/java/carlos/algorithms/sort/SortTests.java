package carlos.algorithms.sort;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public abstract class SortTests {

    abstract void sort(List<Integer> elements);

    @Test
    void testSortEmptyList() {
        List<Integer> elements = new ArrayList<>(List.of());
        sort(elements);
        assertEquals(new ArrayList<>(new ArrayList<>(new ArrayList<>(List.of()))), elements);
    }

    @Test
    void testSortOneElementList() {
        List<Integer> elements = new ArrayList<>(new ArrayList<>(List.of(5)));
        sort(elements);
        assertEquals(new ArrayList<>(new ArrayList<>(List.of(5))), elements);
    }

    @Test
    void testSortSortedList() {
        List<Integer> elements = new ArrayList<>(List.of(1, 2, 3, 4, 5));
        sort(elements);
        assertEquals(new ArrayList<>(List.of(1, 2, 3, 4, 5)), elements);
    }

    @Test
    void testSortReverseSortedList() {
        List<Integer> elements = new ArrayList<>(List.of(5, 4, 3, 2, 1));
        sort(elements);
        assertEquals(new ArrayList<>(List.of(1, 2, 3, 4, 5)), elements);
    }

    @Test
    void testSortUnsortedList() {
        List<Integer> elements = new ArrayList<>(List.of(3, 5, 1, 4, 2));
        sort(elements);
        assertEquals(new ArrayList<>(List.of(1, 2, 3, 4, 5)), elements);
    }

    @Test
    void testSortListWithDuplicates() {
        List<Integer> elements = new ArrayList<>(List.of(3, 5, 1, 4, 2, 3));
        sort(elements);
        assertEquals(new ArrayList<>(List.of(1, 2, 3, 3, 4, 5)), elements);
    }
}
