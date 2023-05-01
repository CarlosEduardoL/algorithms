def generate_sort_tests(sort_function):
    def test_sort_empty_list(self):
        # Test sorting an empty list
        elements = []
        sort_function(elements)
        self.assertEqual(elements, [])

    def test_sort_one_element_list(self):
        # Test sorting a list with one element
        elements = [5]
        sort_function(elements)
        self.assertEqual(elements, [5])

    def test_sort_sorted_list(self):
        # Test sorting a list that is already sorted
        elements = [1, 2, 3, 4, 5]
        sort_function(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_reverse_sorted_list(self):
        # Test sorting a list that is sorted in reverse order
        elements = [5, 4, 3, 2, 1]
        sort_function(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_unsorted_list(self):
        # Test sorting a list that is unsorted
        elements = [3, 5, 1, 4, 2]
        sort_function(elements)
        self.assertEqual(elements, [1, 2, 3, 4, 5])

    def test_sort_list_with_duplicates(self):
        # Test sorting a list with duplicate elements
        elements = [3, 5, 1, 4, 2, 3]
        sort_function(elements)
        self.assertEqual(elements, [1, 2, 3, 3, 4, 5])

    return [
        test_sort_empty_list,
        test_sort_one_element_list,
        test_sort_sorted_list,
        test_sort_reverse_sorted_list,
        test_sort_unsorted_list,
        test_sort_list_with_duplicates
    ]
