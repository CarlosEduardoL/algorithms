## Algorithms and Data Structures Every Programmer Should Know

This repository contains implementations of several algorithms and data structures that every programmer should know, in various programming languages. The content is heavily influenced by the book "40 Algorithms Every Programmer Should Know".

The main purpose of this repository is to provide implementations of these algorithms and data structures in multiple programming languages for educational purposes and to serve as a reference for others who want to learn about these topics.

## Algorithms

### Sort Algorithms

- [x] Bubble sort

- [ ] Merge Sort

- [ ] Insertion Sort

- [ ] SHell Sort

- [ ] Selection Sort

### Searching

- [ ] Linear search
- [ ] Binary search
- [ ] Interpolation Search

### Graphs

- [ ] Breadth-first search
- [ ] Depth-first search

## Languages

Currently, the following programming languages are supported:

- Rust
- Python
- Nim
- Go
- Kotlin
- Java
- Groovy

## Getting Started

To use these algorithms in your project, you can copy the implementation of the algorithm you need from the source files in the corresponding language directory. Each implementation is self-contained and should work out of the box, provided you have the necessary dependencies installed.

Alternatively, you can build the project for the language of your choice and use the resulting library file or executable in your project.

## Contributing

If you would like to contribute to this project, you are welcome to do so! Here are the general steps to follow:

1. Fork the repository
2. Implement the new algorithm or add a new implementation of an existing algorithm in the language of your choice, following the directory structure and file naming conventions used in the repository.
3. Write unit tests for your implementation and add them to the `tests` directory in the corresponding language subdirectory. The test file name should correspond to the implementation file name, with the suffix `_test` added. For example, if you add a new implementation of Bubble Sort in Rust called `bubble_sort.rs`, you should add a corresponding test file called `bubble_sort_test.rs` in the `tests` directory.
4. Update the README.md file to include a description of your new algorithm or implementation, along with usage instructions and any other relevant information.
5. Ensure that all existing unit tests still pass by running the test suite for the corresponding language. If any tests fail, fix the issue before submitting your pull request.
6. Submit a pull request with your changes and a description of what you've added or changed.

Before submitting a pull request, please make sure your changes adhere to the following guidelines:

- Code should be well-documented and follow the relevant style guidelines for the language being used.
- All new code should be covered by unit tests.
- Your pull request should not break any existing functionality or tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
