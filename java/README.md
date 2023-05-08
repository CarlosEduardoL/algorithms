# [Java](https://www.java.com/)

This directory contains Java implementations of various algorithms and data structures.

## algorithms

### Sort Algorithms

- [x] Bubble sort: A simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.
- [x] Insertion Sort: A simple sorting algorithm that builds the final sorted array one item at a time. It is much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.
- [x] Merge Sort: An efficient, general-purpose, comparison-based sorting algorithm. Most implementations produce a stable sort, which means that the order of equal elements is preserved.
- [ ] Shell Sort: An in-place comparison sort. It can be seen as either a generalization of sorting by exchange (bubble sort) or sorting by insertion (insertion sort).
- [ ] Quick Sort: An efficient sorting algorithm. When implemented well, it can be about two or three times faster than its main competitors, merge sort and heapsort.

### Searching

- [ ] Linear search: A method for finding an element within a list. It sequentially checks each element of the list until a match is found or the whole list has been searched.
- [ ] Binary search: A search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array; if they are unequal, the half in which the target cannot lie is eliminated and the search continues on the remaining half until it is successful.
- [ ] Interpolation Search: An algorithm for searching for a given key value in an indexed array that has been ordered by numerical values assigned to the keys (key values).

### Graphs

- [ ] Breadth-first search: An algorithm for traversing or searching tree or graph data structures. It starts at the tree root (or some arbitrary node of a graph) and explores all of the neighbor nodes at the present depth level before moving on to the nodes at the next depth level.
- [ ] Depth-first search: An algorithm for traversing or searching tree or graph data structures. The algorithm starts at the root node (selecting some arbitrary node as the root node in the case of a graph) and explores as far as possible along each branch before backtracking.
- [ ] Dijkstra’s Algorithm: An algorithm for finding the shortest paths between nodes in a graph. It was conceived by computer scientist Edsger W. Dijkstra in 1956 and published three years later.
- [ ] Bellman-Ford Algorithm: An algorithm that computes shortest paths from a single source vertex to all of the other vertices in a weighted digraph.

## Data Structures

### Linear structures

- [ ] Linked Lists: A list in which each element is a node that contains a value and a reference to the next node in the list.

- [ ] Array Lists: A list implemented using an array that dynamically resizes as elements are added or removed.

- [ ] Stacks: Elements are added and removed from the same end, following the “last in, first out” (LIFO) principle.

- [ ] Queues: Elements are added to the end and removed from the beginning, following the “first in, first out” (FIFO) principle.

### Non-linear structures: Elements are not organized in a linear sequence.

- [ ] Binary Trees: A tree in which each node has at most two children.

- [ ] Binary Search Trees: A binary tree in which the left subtree of a node contains only nodes with keys less than the node’s key, and the right subtree contains only nodes with keys greater than the node’s key.

- [ ] Graphs: Elements are organized as nodes connected by edges.

### Collections: Elements are organized in a collection.

- [ ] Sets: A collection of unique elements, i.e., there are no repeated elements.

- [ ] Maps: A collection of key-value pairs where each key is unique.

## Running Tests

To run the tests for this project, you can use the following command:

```bash
./gradlew test
```

This will run all the tests for the project.

## Contributing

If you would like to contribute to this project, you are welcome to do so! Please follow the steps listed in the main README file.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
