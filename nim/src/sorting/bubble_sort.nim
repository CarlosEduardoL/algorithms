type Greater[T] = concept x, y
  x > y is bool

## Sorts the input list in ascending order using the Bubble Sort algorithm in-place.
## Bubble Sort makes n-1 passes, where in each pass, it swaps all the
## elements from 0 to k (where k is n - the pass number), moving the biggest to the end.
## It has a worst-case time complexity of O(n^2).
##
## :param elements: The array of elements to sort.
## :type elements: var openArray[T]
##
proc bubble_sort*[T: Greater](elements: var openArray[T]) =
  if elements.len <= 1: return

  let lastIdx: int = elements.len - 1

  # Iterate over the list from 0 to n-i-1
  for i in lastIdx.countdown(1):
    var swapped = false
    for j in 0 .. i-1:
      # If adjacent elements are in the wrong order, swap them
      if elements[j] > elements[j+1]:
        (elements[j], elements[j+1]) = (elements[j+1], elements[j])
        swapped = true
    # If no swaps occurred in the current pass, the openArray is already sorted
    if not swapped:
      break