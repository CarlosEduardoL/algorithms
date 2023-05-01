import unittest
from sort.bubble_sort import bubble_sort_in_place
from sort.insertion_sort import insertion_sort_in_place
from sort.merge_sort import merge_sort_in_place
from tests.test_sort.generator import generate_sort_tests


class TestSortFunctions(unittest.TestCase):
    pass


sort_functions = [
    bubble_sort_in_place,
    insertion_sort_in_place,
    merge_sort_in_place
]

for sort_function in sort_functions:
    tests = generate_sort_tests(sort_function)
    for test in tests:
        setattr(TestSortFunctions, test.__name__ + sort_function.__name__, test)
