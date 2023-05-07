use std::ops::{Index, IndexMut};

pub trait List<T>: Index<usize, Output=T> + IndexMut<usize, Output=T> {
    /// Returns the element at a specific index in the list.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to return.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the element at the specified index,
    /// or None if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self[index])
        } else {
            None
        }
    }

    /// Adds an element to the front of the list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to add to the front of the list.
    fn push_front(&mut self, data: T);

    /// Adds an element to the back of the list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to add to the back of the list.
    fn push_back(&mut self, data: T);

    /// Removes and returns the first element of the list.
    ///
    /// # Returns
    ///
    /// An option containing the first element of the list,
    /// or None if the list is empty.
    fn pop_front(&mut self) -> Option<T>;

    /// Removes and returns the last element of the list.
    ///
    /// # Returns
    ///
    /// An option containing the last element of the list,
    /// or None if the list is empty.
    fn pop_back(&mut self) -> Option<T>;

    /// Returns the number of elements in the list.
    ///
    /// # Returns
    ///
    /// The number of elements in the list.
    fn len(&self) -> usize;

    /// Checks if the list is empty.
    ///
    /// # Returns
    ///
    /// True if the list is empty, false otherwise.
    fn is_empty(&self) -> bool;
}

macro_rules! list_tests {
    ($list_maker:expr) => {
        #[test]
        fn test_push_pop() {
            let mut list = $list_maker();

            list.push_front(1);
            list.push_front(2);
            list.push_front(3);

            assert_eq!(list.pop_front(), Some(3));
            assert_eq!(list.pop_front(), Some(2));
            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_front(), None);

            list.push_back(4);
            list.push_back(5);
            list.push_back(6);

            assert_eq!(list.pop_back(), Some(6));
            assert_eq!(list.pop_back(), Some(5));
            assert_eq!(list.pop_back(), Some(4));
            assert_eq!(list.pop_back(), None);
        }

        #[test]
        fn test_len_is_empty() {
            let mut list = $list_maker();

            assert_eq!(list.len(), 0);
            assert!(list.is_empty());

            list.push_front(1);
            list.push_front(2);
            list.push_front(3);

            assert_eq!(list.len(), 3);
            assert!(!list.is_empty());

            list.pop_front();
            list.pop_front();

            assert_eq!(list.len(), 1);
            assert!(!list.is_empty());

            list.pop_front();

            assert_eq!(list.len(), 0);
            assert!(list.is_empty());
        }

        #[test]
        fn test_index() {
            let mut list = $list_maker();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            assert_eq!(list[0], 1);
            assert_eq!(list[1], 2);
            assert_eq!(list[2], 3);
        }

        #[test]
        fn test_iterator() {
            let mut list = $list_maker();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), None);
        }
    };
}

pub mod linked_list;
pub mod array_list;