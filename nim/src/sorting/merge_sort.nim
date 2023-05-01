type Comparable[T] = concept x, y
  x > y is bool

## Sorts the input slice in ascending order using the Merge Sort algorithm.
##
## Here are some steps to understand how merge sort works:
## 1. Divide the array into two halves.
## 2. Recursively sort each half.
## 3. Merge two halves.
##
## One of the main advantages of merge sort is that it has a time complexity of O(n log n),
## which means it can sort large arrays relatively quickly. It is also a stable sort,
## which means that the order of elements with equal values is preserved during the sort.
##
## elements: The slice of elements to sort.
##
## Example:
##
## ```
## var v = [3, 2, 1]
## mergeSort(v)
## assert v == [1, 2, 3]
## ```
proc merge_sort*[T: Comparable](elements: var openArray[T]) =
  if len(elements) <= 1: return
  
  let mid = elements.len div 2

  # Split elements in two half
  var left = elements[0 ..< mid ]
  var right = elements[mid ..< elements.len]

  mergeSort(left)
  mergeSort(right)

  # Merge the two sorted halves into a single sorted slice
  var merged: seq[T] = @[]
  var i = 0
  var j = 0
  while i < left.len and j < right.len:
    if left[i] <= right[j]:
      merged.add(left[i])
      inc i
    else:
      merged.add(right[j])
      inc j
  merged.add(left[i ..< left.len])
  merged.add(right[j ..< right.len])
  # Copy the merged sequence back into the original openArray
  for k in 0..<elements.len:
    elements[k] = merged[k]
