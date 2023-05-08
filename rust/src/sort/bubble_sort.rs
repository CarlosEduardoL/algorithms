/// Sorts the input slice in ascending order using the Bubble Sort algorithm.
///
/// Bubble Sort makes n-1 passes, where in each pass, it swaps all the
/// elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
/// It has a worst-case time complexity of O(n^2).
///
/// # Arguments
///
/// * `elements` - The slice of elements to sort.
///
/// # Example
///
/// ```
/// use rust::sort::bubble_sort::bubble_sort;
/// let mut v = vec![3, 2, 1];
/// bubble_sort(&mut v);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn bubble_sort<T: PartialOrd>(elements: &mut [T]) {
    // A slice with one or zero elements is always sorted.
    if elements.len() <= 1 {
        return;
    }
    let last_index = elements.len() - 1;

    // Iterate over the slice n-1 times
    for k in (0..=last_index).rev() {
        let mut swapped = false;
        for index in 0..k {
            if elements[index] > elements[index + 1] {
                elements.swap(index, index + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

test_sort_function!(bubble_sort);