/// Sorts the input slice in ascending order using the Shell Sort algorithm.
///
/// The Shell Sort algorithm is an optimization of the Insertion Sort algorithm. It works by
/// repeatedly dividing the array into smaller subarrays and sorting them using Insertion Sort.
///
/// # Steps
/// 1. Choose a gap sequence: a series of indices that will divide the array into subarrays.
/// 2. Starting with the largest gap, divide the array into subarrays of that size.
/// 3. Sort each subarray using Insertion Sort.
/// 4. Repeat with the next smallest gap until the gap is 1.
///
/// - Best Case is O(n)
/// - Worst Case is O(n^2)
/// - Average Case is generally believed to be between O(n^1.5) and O(n^1.25)
///
/// # Arguments
///
/// * `elements` - The slice of elements to sort.
///
/// # Example
///
/// ``
/// let mut v = vec![3, 2, 1];
/// shell_sort(&mut v);
/// assert_eq!(v, vec![1, 2, 3]);
/// ``
pub fn shell_sort<T: PartialOrd>(elements: &mut [T]) {
    let len = elements.len();
    if len <= 1 { return; }
    let mut gap = len / 2;
    while gap > 0 {
        // Perform Insertion Sort on each gap-sized subarray
        for i in gap..len {
            let mut j = i;
            // Swap elements until they are in order, with a gap-sized step
            while j >= gap && elements[j - gap] > elements[j] {
                elements.swap(j, j - gap);
                j -= gap;
            }
        }
        // Swap elements until they are in order, with a gap-sized step
        gap /= 2;
    }
}

test_sort_function!(shell_sort);