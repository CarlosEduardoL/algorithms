/// Sorts the input slice in ascending order using the Merge Sort algorithm.
///
/// Here are some steps to understand how merge sort works:
/// 1. Divide the array into two halves.
/// 2. Recursively sort each half.
/// 3. Merge two halves.
///
/// One of the main advantages of merge sort is that it has a time complexity of O(n log n),
/// which means it can sort large arrays relatively quickly. It is also a stable sort,
/// which means that the order of elements with equal values is preserved during the sort.
/// # Arguments
///
/// * `elements` - The slice of elements to sort.
///
/// # Example
///
/// ```
/// use algorithms::sort::mergesort::merge_sort;
/// let mut v = vec![3, 2, 1];
/// merge_sort(&mut v);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn merge_sort<T: Clone + PartialOrd + Copy>(elements: &mut [T]) {
    if elements.len() <= 1 {return;}
    let mut merged = Vec::with_capacity(elements.len());
    let mid = elements.len()/2;
    // Split elements in two half
    let (left, right) = elements.split_at_mut(mid);
    merge_sort(left);
    merge_sort(right);

    // Merge the two sorted halves into a single sorted slice
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    elements.copy_from_slice(&merged);
}

test_sort_function!(merge_sort);