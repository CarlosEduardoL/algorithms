package carlos.algorithms.sort;

import java.util.List;

public final class Insertion {
    /**
     * Sorts the input list in ascending order using the Insertion Sort algorithm.
     * Insertion sort is a simple algorithm that sorts a list of items by repeatedly
     * inserting each item into its correct position within the sorted portion of the list.
     * To do this, we compare the current item with the items to its left until we find its correct position,
     * then we insert it.
     * <p>
     * This process is repeated for each item in the list until they are all in their correct positions.
     * Insertion Sort has a worst-case time complexity of O(n^2).
     *
     * @param elements List of elements to sort in place.
     * @throws IllegalArgumentException If elements is null.
     */
    public static <T extends Comparable<T>> void sort(List<T> elements) {
        if (elements == null) {
            throw new IllegalArgumentException("Input list cannot be null.");
        }

        if (elements.size() <= 1) {
            return;
        }

        for (int i = 1; i < elements.size(); i++) {
            T currentElement = elements.get(i);
            int previousIndex = i - 1;

            while (previousIndex >= 0 && elements.get(previousIndex).compareTo(currentElement) > 0) {
                elements.set(previousIndex + 1, elements.get(previousIndex));
                previousIndex--;
            }

            elements.set(previousIndex + 1, currentElement);
        }
    }
}
