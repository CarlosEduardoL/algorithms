/// Sorts a mutable slice of elements using the QuickSort algorithm.
///
/// QuickSort is a divide-and-conquer sorting algorithm that works by
/// partitioning the input slice into two sub-slices around a pivot element,
/// such that all elements in the left sub-slice are less than the pivot
/// element and all elements in the right sub-slice are greater than or equal
/// to the pivot element. The two sub-slices are then sorted recursively.
///
/// # Best Case
///
/// The best case for QuickSort occurs when the pivot element is always chosen
/// as the median element of the slice. In this case, the slice is always
/// partitioned into two sub-slices of equal size, and the depth of recursion
/// is `O(log n)`. The time complexity of QuickSort in the best case is `O(n log n)`.
///
/// # Worst Case
///
/// The worst case for QuickSort occurs when the pivot element is always chosen
/// as the smallest or largest element of the slice. In this case, one of the
/// sub-slices is always empty, and the depth of recursion is `O(n)`. The time
/// complexity of QuickSort in the worst case is `O(n^2)`.
///
/// # Examples
///
/// ```
/// use rust::sort::quicksort::quicksort;
/// let mut arr = [5, 2, 1, 4, 3];
/// quicksort(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```
pub fn quicksort<T: PartialOrd>(arr: &mut [T]) {
    // Base case: if the slice has length less than or equal to 1,
    // it is already sorted.
    if arr.len() <= 1 {
        return;
    }

    // Partition the slice around a pivot element.
    let pivot_index = partition(arr);

    // Recursively sort the left and right sub-slices.
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

/// Partitions a mutable slice of elements around a pivot element using
/// the Lomuto partition scheme.
///
/// The Lomuto partition scheme works by maintaining two indices `i` and `j`.
/// All elements before index `i` are less than the pivot element, and all
/// elements between indices `i` and `j` are greater than or equal to the pivot
/// element. The function iterates over the elements of the slice from index 0
/// to index `pivot_index - 1`. If an element at index `j` is less than the pivot
/// element, it is swapped with the element at index `i`, and `i` is incremented.
///
/// After all elements have been processed, the pivot element is swapped with
/// the element at index `i`. This places the pivot element at its final position
/// in the sorted slice. The function then returns the index of the pivot element.
fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    // Choose the last element as the pivot element.
    let pivot_index = arr.len() - 1;
    let mut i = 0;

    // Iterate over elements from index 0 to index `pivot_index - 1`.
    for j in 0..pivot_index {
        // If an element at index `j` is less than the pivot element,
        // swap it with the element at index `i` and increment `i`.
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // Swap the pivot element with the element at index `i`.
    arr.swap(i, pivot_index);

    // Return index of pivot element.
    i
}

test_sort_function!(quicksort);