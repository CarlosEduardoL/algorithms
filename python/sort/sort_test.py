import unittest
from bubble_sort import bubble_sort_in_place


class TestBubbleSort(unittest.TestCase):
    def test_sort_empty_list(self):
        # Test sorting an empty list
        elements = []
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [])

    def test_sort_one_element_list(self):
        # Test sorting a list with one element
        elements = [5]
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [5])

    def test_sort_sorted_list(self):
        # Test sorting a list that is already sorted
        elements = [1, 2, 3, 4, 5]
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_reverse_sorted_list(self):
        # Test sorting a list that is sorted in reverse order
        elements = [5, 4, 3, 2, 1]
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_unsorted_list(self):
        # Test sorting a list that is unsorted
        elements = [3, 5, 1, 4, 2]
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_list_with_duplicates(self):
        # Test sorting a list with duplicate elements
        elements = [3, 5, 1, 4, 2, 3]
        bubble_sort_in_place(elements)
        self.assertEqual(elements, [1, 2, 3, 3, 4, 5])

    def test_raises_type_error(self):
        # Test that a TypeError is raised if the input is not a list
        with self.assertRaises(TypeError):
            bubble_sort_in_place("not a list")


if __name__ == '__main__':
    unittest.main()
