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
/// let mut v = vec![3, 2, 1];
/// bubble_sort(&mut v);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn bubble_sort<T: PartialOrd>(elements: &mut [T]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_list() {
        let mut elements: [i32; 0] = [];
        bubble_sort(&mut elements);
        assert_eq!(elements, []);
    }

    #[test]
    fn sort_one_element_list() {
        let mut elements = [5];
        bubble_sort(&mut elements);
        assert_eq!(elements, [5]);
    }

    #[test]
    fn sort_sorted_list() {
        let mut elements = [1, 2, 3, 4, 5];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_reverse_sorted_list() {
        let mut elements = [5, 4, 3, 2, 1];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_unsorted_list() {
        let mut elements = [3, 5, 1, 4, 2];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_list_with_duplicates() {
        let mut elements = [3, 5, 1, 4, 2, 3];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1, 2, 3, 3, 4, 5]);
    }
}