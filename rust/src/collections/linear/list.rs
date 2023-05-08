use std::ops::{Index, IndexMut};

pub trait List<T>: Index<usize, Output=T> + IndexMut<usize, Output=T> {
    /// Returns the element at a specific index in the linear.
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

    /// Adds an element to the front of the linear.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to add to the front of the linear.
    fn push_front(&mut self, data: T);

    /// Adds an element to the back of the linear.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to add to the back of the linear.
    fn push_back(&mut self, data: T);

    /// Removes and returns the first element of the linear.
    ///
    /// # Returns
    ///
    /// An option containing the first element of the linear,
    /// or None if the linear is empty.
    fn pop_front(&mut self) -> Option<T>;

    /// Removes and returns the last element of the linear.
    ///
    /// # Returns
    ///
    /// An option containing the last element of the linear,
    /// or None if the linear is empty.
    fn pop_back(&mut self) -> Option<T>;

    /// Returns the number of elements in the linear.
    ///
    /// # Returns
    ///
    /// The number of elements in the linear.
    fn len(&self) -> usize;

    /// Checks if the linear is empty.
    ///
    /// # Returns
    ///
    /// True if the linear is empty, false otherwise.
    fn is_empty(&self) -> bool;
}