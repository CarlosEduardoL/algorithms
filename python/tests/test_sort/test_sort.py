import unittest
from sort.bubble_sort import bubble_sort_in_place
from sort.insertion_sort import insertion_sort
from sort.merge_sort import merge_sort
from sort.quick_sort import quicksort
from sort.shell_sort import shell_sort
from tests.test_sort.generator import generate_sort_tests


class TestSortFunctions(unittest.TestCase):
    pass


sort_functions = [
    bubble_sort_in_place,
    insertion_sort,
    merge_sort,
    shell_sort,
    quicksort
]

for sort_function in sort_functions:
    tests = generate_sort_tests(sort_function)
    for test in tests:
        setattr(TestSortFunctions, test.__name__ + "_" + sort_function.__name__, test)
