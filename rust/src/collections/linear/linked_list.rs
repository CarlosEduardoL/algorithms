use std::ops::{Index, IndexMut};
use std::ptr;
use crate::collections::linear::list::List;
use crate::mem_utils::allocate;

/// Each node in the linked list contains some data and a pointer to the next node.
struct Node<T> {
    data: T,
    next: *mut Node<T>, // raw pointer to the next node
}

impl<T> Node<T> {
    /// Create a new node with given data and a null pointer to the next node.
    fn new(data: T) -> Self {
        Node { data, next: ptr::null_mut() }
    }
}

/// The linked list struct contains a raw pointer to the head node, a raw pointer to the tail node,
/// and the length of the linked list.
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    // Create a new linked list with null pointers to the head and tail nodes and length 0.
    pub fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
}

/// A linked list is a linear data structure where each element is a separate object called a node.
/// Each node contains a value and a reference to the next node in the list.
/// The first node is called the head and the last node is called the tail.
/// Linked lists can be used to implement various abstract data types such as stacks, queues, and associative arrays.
impl<T> List<T> for LinkedList<T> {
    /// Adds an element to the front of the list. Time complexity: O(1).
    fn push_front(&mut self, data: T) {
        let raw_node = allocate(Node::new(data));

        // Set the new node's next pointer to the current head, and update the head pointer to the new node.
        unsafe {
            (*raw_node).next = self.head;
            self.head = raw_node;

            // If the tail pointer is null, update it to the new node.
            if self.tail.is_null() {
                self.tail = raw_node;
            }

            self.len += 1;
        }
    }

    /// Adds an element to the back of the list. Time complexity: O(1).
    fn push_back(&mut self, data: T) {
        let raw_node = allocate(Node::new(data));

        // If the tail pointer is null, update the head pointer to the new node.
        // Otherwise, set the previous tail's next pointer to the new node.
        unsafe {
            if self.tail.is_null() {
                self.head = raw_node;
            } else {
                (*self.tail).next = raw_node;
            }

            // Update the tail pointer to the new node.
            self.tail = raw_node;
            self.len += 1;
        }
    }

    /// Removes and returns the first element of the list. Time complexity: O(1).
    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        // Remove the current head node and update the head pointer to the next node.
        let old_head = unsafe { Box::from_raw(self.head) };
        self.head = old_head.next;

        // If the head pointer is null, update the tail pointer to null.
        if self.head.is_null() {
            self.tail = ptr::null_mut();
        }

        self.len -= 1;
        Some(old_head.data)
    }

    /// Removes and returns the last element of the list. Time complexity: O(n).
    fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        if self.head == self.tail {
            return self.pop_front();
        }

        let mut prev = ptr::null_mut();
        let mut node = self.head;

        // Traverse the list to find the second-to-last node
        while !unsafe { (*node).next.is_null() } {
            prev = node;
            node = unsafe { (*node).next };
        }

        // Update the tail pointer and return the data from the last node
        unsafe {
            (*prev).next = ptr::null_mut();
            self.tail = prev;
            self.len -= 1;
            Some(Box::from_raw(node).data)
        }
    }

    /// Returns the number of elements in the list. Time complexity: O(1).
    fn len(&self) -> usize {
        self.len
    }

    /// Checks if the list is empty. Time complexity: O(1).
    fn is_empty(&self) -> bool {
        self.head.is_null()
    }
}

impl<T> Drop for LinkedList<T> {
    /// Drops all the elements in the `LinkedList`
    fn drop(&mut self) {
        let mut node = self.head;
        while !node.is_null() {
            let next = unsafe { (*node).next };
            drop(unsafe { Box::from_raw(node) });
            node = next;
        }
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    /// Gets the element at the given `index`. Time complexity: O(n).
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Index out of bounds");
        }

        let mut node = self.head;
        for _ in 0..index {
            unsafe {
                node = (*node).next;
            }
        }

        unsafe {
            &(*node).data
        }
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    /// Gets the element at the given `index`. Time complexity: O(n).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len() {
            panic!("Index out of bounds");
        }

        let mut node = self.head;
        for _ in 0..index {
            unsafe {
                node = (*node).next;
            }
        }

        unsafe {
            &mut (*node).data
        }
    }
}

/// An iterator over the elements in a `LinkedList`
pub struct LinkedListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    /// Returns the next element in the `LinkedList`
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.data
        })
    }
}

impl<T> LinkedList<T> {
    #[allow(dead_code)]
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter { next: unsafe { self.head.as_ref() } }
    }
}

list_tests!(LinkedList::new);