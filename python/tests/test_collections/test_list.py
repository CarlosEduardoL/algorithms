import unittest

from ds.LinkedList import LinkedList


class TestLinkedList(unittest.TestCase):
    def test_push_front(self):
        linked_list = LinkedList()
        linked_list.push_front(3)
        self.assertEqual(linked_list.head.data, 3)
        linked_list.push_front(2)
        self.assertEqual(linked_list.head.data, 2)
        self.assertEqual(linked_list.head.next.data, 3)

    def test_push_back(self):
        linked_list = LinkedList()
        linked_list.push_back(1)
        self.assertEqual(linked_list.head.data, 1)
        linked_list.push_back(2)
        self.assertEqual(linked_list.head.next.data, 2)

    def test_pop_front(self):
        linked_list = LinkedList()
        self.assertIsNone(linked_list.pop_front())
        linked_list.push_front(1)
        self.assertEqual(linked_list.pop_front(), 1)
        self.assertIsNone(linked_list.pop_front())

    def test_pop_back(self):
        linked_list = LinkedList()
        self.assertIsNone(linked_list.pop_back())
        linked_list.push_back(1)
        self.assertEqual(linked_list.pop_back(), 1)
        self.assertIsNone(linked_list.pop_back())

    def test_len(self):
        linked_list = LinkedList()
        self.assertEqual(linked_list.len(), 0)
        linked_list.push_front(1)
        self.assertEqual(linked_list.len(), 1)
        linked_list.push_back(2)
        self.assertEqual(linked_list.len(), 2)

    def test_is_empty(self):
        linked_list = LinkedList()
        self.assertTrue(linked_list.is_empty())
        linked_list.push_front(1)
        self.assertFalse(linked_list.is_empty())

    def test_getitem(self):
        linked_list = LinkedList()
        with self.assertRaises(IndexError):
            _ = linked_list[0]
        linked_list.push_back(1)
        self.assertEqual(linked_list[0], 1)

    def test_setitem(self):
        linked_list = LinkedList()
        with self.assertRaises(IndexError):
            linked_list[0] = 1
        linked_list.push_back(1)
        linked_list[0] = 2
        self.assertEqual(linked_list[0], 2)
