class Node:
    def __init__(self, data):
        """
        Initializes a new node with the given data and a null pointer to the next node.
        :param data: The data to be stored in the node.
        """
        self.data = data
        self.next = None


class LinkedList:
    def __init__(self):
        """
        Initializes a new linked list with a null pointer to the head node.
        """
        self.head = None
        self.size = 0

    def push_front(self, data):
        """
        Adds an element to the front of the list. Time complexity: O(1).
        :param data: The data to be added to the front of the list.
        """
        new_node = Node(data)
        new_node.next = self.head
        self.head = new_node
        self.size += 1

    def push_back(self, data):
        """
        Adds an element to the back of the list. Time complexity: O(n).
        :param data: The data to be added to the back of the list.
        """
        new_node = Node(data)
        if self.head is None:
            self.head = new_node
            self.size += 1
            return
        last = self.head
        while last.next:
            last = last.next
        last.next = new_node
        self.size += 1

    def pop_front(self):
        """
        Removes and returns the first element of the list. Time complexity: O(1).
        :return: The first element of the list or None if the list is empty.
        """
        if self.head is None:
            return None
        temp = self.head
        self.head = temp.next
        temp.next = None
        self.size -= 1
        return temp.data

    def pop_back(self):
        """
        Removes and returns the last element of the list. Time complexity: O(n).
        :return: The last element of the list or None if the list is empty.
        """
        if self.head is None:
            return None
        if self.head.next is None:
            temp = self.head
            self.head = None
            return temp.data
        second_last = self.head
        while second_last.next.next:
            second_last = second_last.next
        temp = second_last.next
        second_last.next = None
        self.size -= 1
        return temp.data

    def len(self) -> int:
        """
        Returns the number of elements in the list. Time complexity: O(1).
        :return: The number of elements in the list.
        """
        return self.size

    def is_empty(self):
        """
        Checks if the list is empty. Time complexity: O(1).
        :return: True if the list is empty, False otherwise.
        """
        return not bool(self.len())

    def __getitem__(self, index):
        """
        Gets the element at the given index. Time complexity: O(n).
        :param index: The index of the element to get.
        :return: The element at the given index.
        :raises IndexError: If the index is out of bounds.
        """
        if index >= self.len():
            raise IndexError("Index out of bounds")
        current_node = self.head
        for _ in range(index):
            current_node = current_node.next
        return current_node.data

    def __setitem__(self, index, value):
        """
        Sets the element at the given index. Time complexity: O(n).
        :param index: The index of the element to set.
        :param value: The value to set at the given index.
        :raises IndexError: If the index is out of bounds.
        """
        if index >= self.len():
            raise IndexError("Index out of bounds")
        current_node = self.head
        for _ in range(index):
            current_node = current_node.next
        current_node.data = value

    def __iter__(self):
        """
        Returns an iterator over the elements in the list. Time complexity: O(1).
        :return: An iterator over the elements in the list.
        """
        current_node = self.head
        while current_node:
            yield current_node.data
            current_node = current_node.next
