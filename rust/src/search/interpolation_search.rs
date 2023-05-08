/// Performs an interpolation search on a slice of elements to find the index of a given item.
///
/// # Arguments
///
/// * `elements` - A slice of elements to search through.
/// * `item` - The item to search for in the slice of elements.
///
/// # Returns
///
/// * `Option<usize>` - The index of the item in the slice if it is found, or `None` if it is not found.
///
/// # Complexity
///
/// * The average time complexity of this function is O(log log n), where n is the number of elements in the slice.
/// * The worst-case time complexity of this function is O(n), where n is the number of elements in the slice.
pub fn interpolation_search<T: num_traits::Num + num_traits::AsPrimitive<usize> + PartialOrd>(elements: &[T], item: &T) -> Option<usize> {
    // Return None if the slice is empty
    if elements.is_empty() { return None; }

    // Initialize low and high indices
    let (mut low, mut high) = (0, elements.len() - 1);

    // While the low index is less than or equal to the high index and the item is within the range of elements
    while low <= high && *item >= elements[low] && *item <= elements[high] {
        // If low and high indices are equal
        if low == high {
            // If the element at the low index is equal to the item, return Some(low)
            if elements[low] == *item { return Some(low); }
            // Otherwise, return None
            return None;
        }

        // Calculate the position to check based on interpolation
        let pos: usize = low + (((high - low) / (elements[high].as_() - elements[low].as_())) * ((*item).as_() - elements[low].as_()));

        // If the element at pos is equal to the item, return Some(pos)
        if elements[pos] == *item {
            return Some(pos);
        }

        // If the element at pos is less than the item, update low to pos + 1
        if elements[pos] < *item {
            low = pos + 1;
        } else {
            // Otherwise, update high to pos - 1
            high = pos - 1;
        }
    }

    // Return None if the item was not found
    None
}

generate_search_tests!(interpolation_search);