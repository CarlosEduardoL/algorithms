import macro_gen_test
import sorting/bubble_sort
import unittest

suite "Bubble Sort":
  macro_gen_test.generate_sort_tests("bubble_sort")