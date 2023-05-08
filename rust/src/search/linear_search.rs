/// Searches for an item in a slice using linear search.
///
/// This function takes a slice of elements and an item to search for.
///
/// or `None` if the item is not found.
///
/// The time complexity of this function is O(n), where n is the number of elements in the slice.
///
/// # Examples
///
/// ```
/// use algorithms::search::linear_search::linear_search;
/// let numbers = [1, 2, 3, 4, 5];
/// let result = linear_search(&numbers, &3);
/// assert_eq!(result, Some(2));
///
/// let result = linear_search(&numbers, &6);
/// assert_eq!(result, None);
/// ```
pub fn linear_search<T: PartialEq>(elements: &[T], item: &T) -> Option<usize> {
    for (index, element) in elements.iter().enumerate() {
        if *element == *item {
            return index.into();
        }
    }
    None
}

generate_search_tests!(linear_search);