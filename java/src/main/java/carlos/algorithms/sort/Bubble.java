package carlos.algorithms.sort;

import java.util.Collections;
import java.util.List;

public final class Bubble {
    /**
     * Sorts the input list in ascending order using the Bubble Sort algorithm in-place.
     * <p>
     * Bubble Sort makes n-1 passes, where in each pass, it swaps all the
     * elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
     * It has a worst-case time complexity of O(n^2).
     *
     * @param elements List of elements to sort.
     * @param <T>      Any comparable type
     * @throws IllegalArgumentException If the input list is null.
     */
    public static <T extends Comparable<T>> void sort(List<T> elements) {
        if (elements == null) {
            throw new IllegalArgumentException("Input list cannot be null.");
        }

        if (elements.size() <= 1) {
            return;
        }

        int lastIndex = elements.size() - 1;
        for (int k = lastIndex; k >= 0; k--) {
            // Keeps track of whether any swaps have occurred in this pass
            var swapped = false;
            for (int index = 0; index < k; index++)
                if (elements.get(index).compareTo(elements.get(index + 1)) > 0) {
                    Collections.swap(elements, index, index + 1);
                    swapped = true;
                }
            if (!swapped) return; // No swaps occurred in this pass, so the list is sorted
        }
    }
}
