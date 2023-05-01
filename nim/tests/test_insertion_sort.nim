import unittest
import macro_gen_test
import sorting/insertion_sort

# Define a test suite for the insertion_sort function
suite "Insertion Sort":
    macro_gen_test.generate_sort_tests("insertion_sort")