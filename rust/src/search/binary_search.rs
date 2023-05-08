/// Searches for an item in a sorted slice using binary search.
///
/// This function takes a sorted slice of elements and an item to search for.
/// It returns the index of the first occurrence of the item in the slice,
/// or `None` if the item is not found.
///
/// The time complexity of this function is O(log n), where n is the number of elements in the slice.
///
/// # Examples
///
/// ```
/// use algorithms::search::binary_search::binary_search;
/// let numbers = [1, 2, 3, 4, 5];
/// let result = binary_search(&numbers, &3);
/// assert_eq!(result, Some(2));
///
/// let result = binary_search(&numbers, &6);
/// assert_eq!(result, None);
/// ```
pub fn binary_search<T: PartialOrd + PartialEq>(elements: &[T], item: &T) -> Option<usize> {
    if elements.is_empty() { return None; }
    let (mut first, mut last) = (0, elements.len() - 1);
    while first <= last {
        let mid = (first + last) / 2;
        if elements[mid] == *item {
            return mid.into();
        }
        if *item < elements[mid] {
            last = mid - 1
        } else {
            first = mid + 1
        }
    }
    None
}

generate_search_tests!(binary_search);