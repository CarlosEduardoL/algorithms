package carlos.algorithms.sort;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Merge<T extends Comparable<T>> {

    /**
     * Sorts the input list in ascending order using the Merge Sort algorithm.
     *
     * Here are some steps to understand how merge sort works:
     * 1. Divide the list into two halves.
     * 2. Recursively sort each half.
     * 3. Merge two halves.
     *
     * One of the main advantages of merge sort is that it has a time complexity of O(n log n),
     * which means it can sort large lists relatively quickly. It is also a stable sort,
     * which means that the order of elements with equal values is preserved during the sort.
     *
     * @param <T> The type of elements in the list
     * @param elements The list of elements to sort
     */
    public static <T extends Comparable<T>> void sort(List<T> elements) {
        if (elements.size() <= 1) return;
        int mid = elements.size() / 2;
        // Split elements in two half
        List<T> left = new ArrayList<>(elements.subList(0, mid));
        List<T> right = new ArrayList<>(elements.subList(mid, elements.size()));
        sort(left);
        sort(right);

        // Merge the two sorted halves into a single sorted list
        List<T> merged = new ArrayList<>();
        int i = 0, j = 0;
        while (i < left.size() && j < right.size()) {
            if (left.get(i).compareTo(right.get(j)) <= 0) {
                merged.add(left.get(i));
                i++;
            } else {
                merged.add(right.get(j));
                j++;
            }
        }
        merged.addAll(left.subList(i, left.size()));
        merged.addAll(right.subList(j, right.size()));
        for (int k = 0; k < merged.size(); k++) {
            elements.set(k, merged.get(k));
        }
    }
}
