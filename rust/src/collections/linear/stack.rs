use crate::collections::linear::linked_list::LinkedList;
use crate::collections::linear::list::List;

/// A stack is a linear data structure that follows the Last In First Out (LIFO) principle.
/// Elements are added to the top of the stack and removed from the top of the stack.
///
/// This implementation of a stack uses a linked list as the underlying data structure.
/// Each node in the linked list contains a value and a reference to the next node in the list.
/// The first node is called the head and represents the top of the stack.
///
/// # Examples
///
/// ```
/// use rust::collections::linear::stack::Stack;
/// let mut stack = Stack::new();
///
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
///
/// assert_eq!(stack.pop(), Some(3));
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
#[repr(transparent)]
pub struct Stack<T> {
    list: LinkedList<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn new() -> Self { Self { list: LinkedList::new() } }

    /// Adds an element to the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the stack.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn push(&mut self, value: T) {
        self.list.push_front(value)
    }

    /// Removes and returns the element at the top of the stack.
    ///
    /// # Returns
    ///
    /// The element at the top of the stack, or `None` if the stack is empty.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Returns a reference to the element at the top of the stack without removing it.
    ///
    /// # Returns
    ///
    /// A reference to the element at the top of the stack, or `None` if the stack is empty.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn peek(&self) -> Option<&T> {
        if self.len() <= 0 {
            None
        } else {
            Some(&self.list[0])
        }
    }

    /// Returns the number of elements in the stack.
    ///
    /// # Returns
    ///
    /// The number of elements in the stack.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn len(&self) -> usize { self.list.len() }

    /// Checks if the stack is empty.
    ///
    /// # Returns
    ///
    /// `true` if the stack is empty, `false` otherwise.
    ///
    /// # Complexity
    ///
    /// This operation has a time complexity of O(1).
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn test_len() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_pop_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_push_pop_large() {
        let mut stack = Stack::new();
        for i in 0..1000 {
            stack.push(i);
        }
        for i in (0..1000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
        assert_eq!(stack.pop(), None);
    }
}