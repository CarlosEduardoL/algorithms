use crate::collections::linear::linked_list::LinkedList;
use crate::collections::linear::list::List;

/// A queue is a linear data structure that follows the First In First Out (FIFO) principle.
/// Elements are added to the end of the queue and removed from the front of the queue.
///
/// This implementation of a queue uses a linked list as the underlying data structure.
/// Each node in the linked list contains a value and a reference to the next node in the list.
/// The first node is called the head and represents the front of the queue.
/// The last node is called the tail and represents the end of the queue.
///
/// # Examples
///
/// ```
/// use rust::collections::linear::queue::Queue;
/// let mut queue = Queue::new();
///
/// queue.enqueue(1);
/// queue.enqueue(2);
/// queue.enqueue(3);
///
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.dequeue(), Some(2));
/// assert_eq!(queue.dequeue(), Some(3));
/// assert_eq!(queue.dequeue(), None);
/// ```
pub struct Queue<T> {
    list: LinkedList<T>,
}

impl<T> Queue<T> {
    /// Creates a new, empty queue.
    ///
    /// Time complexity: O(1)
    pub fn new() -> Queue<T> {
        Self { list: LinkedList::new() }
    }

    /// Adds an element to the end of the queue.
    ///
    /// Time complexity: O(1)
    pub fn enqueue(&mut self, value: T) {
        self.list.push_back(value)
    }

    /// Removes and returns the element at the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// Time complexity: O(1)
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Returns the number of elements in the queue.
    ///
    /// Time complexity: O(1)
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns `true` if the queue is empty and `false` otherwise.
    ///
    /// Time complexity: O(1)
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_len() {
        let mut queue = Queue::new();

        assert_eq!(queue.len(), 0);

        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());

        queue.enqueue(1);

        assert!(!queue.is_empty());
    }
}