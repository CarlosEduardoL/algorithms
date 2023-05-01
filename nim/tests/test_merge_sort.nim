import macro_gen_test
import sorting/merge_sort
import unittest

suite "Bubble Sort":
  macro_gen_test.generate_sort_tests("merge_sort")