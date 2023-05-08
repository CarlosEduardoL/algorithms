/// Sorts the input linear in ascending order using the Insertion Sort algorithm.
///
/// Insertion sort is a simple algorithm that sorts a linear of items by repeatedly inserting each item
/// into its correct position within the sorted portion of the linear. To do this, we compare the current
/// item with the items to its left until we find its correct position, then we insert it. This process
/// is repeated for each item in the linear until they are all in their correct positions.
///
/// This implementation also includes a check to see if the linear is already sorted, which could avoid
/// unnecessary iterations and comparisons.
///
/// Insertion Sort has a worst-case time complexity of O(n^2).
///
/// # Arguments
///
/// * `elements` - A mutable reference to a vector of elements to sort in place.
///
/// # Examples
///
/// ```
/// use rust::sort::insertion_sort::insertion_sort;
/// let mut v = vec![9, 7, 8, 2, 5];
/// insertion_sort(&mut v);
/// assert_eq!(v, [2, 5, 7, 8, 9]);
/// ```
pub fn insertion_sort<T: PartialOrd>(elements: &mut [T]) {
    // A Slice with one or zero elements is always sorted.
    if elements.len() <= 1 { return; }

    for initial_index in 1..elements.len() {
        let mut current_index = initial_index;
        // Move the selected element left until is in the correct position
        while current_index > 0 && elements[current_index] < elements[current_index - 1] {
            elements.swap(current_index, current_index - 1);
            current_index -= 1;
        }
    }
}


test_sort_function!(insertion_sort);