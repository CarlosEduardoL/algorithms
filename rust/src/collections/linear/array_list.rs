use crate::mem_utils::{alloc, dealloc};
use std::{fmt, ptr};
use std::ops::{Index, IndexMut};
use crate::collections::linear::list::List;

/// Set the initial capacity of the ArrayList
const INITIAL_CAPACITY: usize = 2 << 3;

/// Define the ArrayList struct with a buffer pointer, capacity and length
/// An array linear is a dynamic array that can grow or shrink in size as needed.
/// It is implemented as an array with a fixed initial capacity that can be increased when the array becomes full
pub struct ArrayList<T> {
    array: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> ArrayList<T> {
    #[allow(dead_code)]
    // Create a new ArrayList with an initial buffer and capacity
    pub fn new() -> Self {
        let array = unsafe {
            let buffer = alloc(INITIAL_CAPACITY);
            ptr::write_bytes(buffer, 0, INITIAL_CAPACITY);
            buffer
        };
        ArrayList {
            array,
            capacity: INITIAL_CAPACITY,
            len: 0,
        }
    }

    // Grow the ArrayList by doubling its capacity if it's full
    fn grow(&mut self) {
        if self.len == self.capacity {
            let new_capacity = self.capacity * 2;
            let new_buffer = unsafe {
                let new_buffer = alloc(new_capacity);
                ptr::copy_nonoverlapping(self.array, new_buffer, self.len);
                dealloc(self.array, self.capacity);
                new_buffer
            };
            self.array = new_buffer;
            self.capacity = new_capacity;
        }
    }
    // create a new buffer with a smaller capacity and copy the existing elements into it.
    fn shrink(&mut self) {
        if self.len < self.capacity / 2 {
            let new_capacity = self.capacity / 2;
            let new_buffer = unsafe {
                let new_buffer = alloc(new_capacity);
                ptr::copy_nonoverlapping(self.array, new_buffer, self.len);
                dealloc(self.array, self.capacity);
                new_buffer
            };
            self.array = new_buffer;
            self.capacity = new_capacity;
        }
    }
}

// Implement the List trait for ArrayList
impl<T> List<T> for ArrayList<T> {
    /// Add an element to the front of the linear and has a time complexity of O(n) because it needs to
    /// shift all existing elements to the right by one position to make room for the new element at the front.
    fn push_front(&mut self, data: T) {
        self.grow();
        unsafe {
            ptr::copy(self.array, self.array.offset(1), self.len);
            ptr::write(self.array, data);
        }
        self.len += 1;
    }

    /// Add an element to the back of the linear and has an amortized time complexity of O(1) because
    /// it adds the new element at the end of the array. However, if the array is full and needs to be resized, this operation can take O(n) time in the worst case.
    fn push_back(&mut self, data: T) {
        self.grow();
        unsafe { ptr::write(self.array.add(self.len), data) };
        self.len += 1;
    }

    /// Remove and return the first element of the linear and has a time complexity of O(n) because it
    /// needs to shift all existing elements to the left by one position after removing the first element.
    fn pop_front(&mut self) -> Option<T> {
        let result = if self.is_empty() {
            None
        } else {
            let data = unsafe { ptr::read(self.array) };
            unsafe { ptr::copy(self.array.offset(1), self.array, self.len - 1) };
            self.len -= 1;
            Some(data)
        };
        self.shrink();
        result
    }

    /// Remove and return the last element of the linear and has a time complexity of O(1) because it
    /// directly removes the last element in constant time.
    fn pop_back(&mut self) -> Option<T> {
        let result = if self.is_empty() {
            None
        } else {
            let data = unsafe { ptr::read(self.array.add(self.len - 1)) };
            self.len -= 1;
            Some(data)
        };
        self.shrink();
        result
    }

    /// Return the length of the linear and has a time complexity of O(1) because they return the
    /// value of the len field in constant time.
    fn len(&self) -> usize {
        self.len
    }

    /// Check if the list is empty and has a time complexity of O(1) because they return the
    /// value of the len field in constant time.
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// Implement Drop for ArrayList to deallocate its buffer when dropped
impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        unsafe { dealloc(self.array, self.capacity) };
    }
}

// Implement Index for ArrayList to allow indexing into it
impl<T> Index<usize> for ArrayList<T> {
    type Output = T;
    /// Gets the element at the given `index`. Time complexity: O(1).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        unsafe { &*self.array.add(index) }
    }
}

// Implement IndexMut for ArrayList to allow mutable indexing into it
impl<T> IndexMut<usize> for ArrayList<T> {
    /// Gets the element at the given `index` as mut. Time complexity: O(1).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len);
        unsafe { &mut *self.array.add(index) }
    }
}

impl<T: fmt::Debug> fmt::Debug for ArrayList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// Define an iterator for ArrayList
impl<T> ArrayList<T> {
    #[allow(dead_code)]
    pub fn iter(&self) -> ArrayListIter<'_, T> {
        ArrayListIter {
            list: self,
            index: 0,
        }
    }
}

pub struct ArrayListIter<'a, T> {
    list: &'a ArrayList<T>,
    index: usize,
}

impl<'a, T> Iterator for ArrayListIter<'a, T> {
    type Item = &'a T;

    // Return the next element in the iterator
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.list.get(self.index) {
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod array_list_test {
    use crate::collections::linear::array_list::{ArrayList, INITIAL_CAPACITY};
    use crate::collections::linear::list::List;

    #[test]
    fn test_grow_shrink() {
        let mut list = ArrayList::new();

        // Check initial capacity
        assert_eq!(list.capacity, INITIAL_CAPACITY);

        // Push elements to grow the array
        for i in 0..INITIAL_CAPACITY {
            list.push_back(i);
            assert_eq!(list.capacity, INITIAL_CAPACITY);
        }

        // Push one more element to trigger a grow
        list.push_back(INITIAL_CAPACITY);
        assert_eq!(list.capacity, INITIAL_CAPACITY * 2);

        list.pop_back();
        // Pop one more element to trigger a shrink
        list.pop_back();
        assert_eq!(list.capacity, INITIAL_CAPACITY);

        // Pop elements to shrink the array
        for _ in 0..(list.capacity / 2 - 1) {
            list.pop_back();
            assert_eq!(list.capacity, INITIAL_CAPACITY);
        }
    }
}

list_tests!(ArrayList::new);