macro_rules! test_sort_function {
    ($sort_function:path) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn sort_empty_list() {
                let mut elements: [i32; 0] = [];
                $sort_function(&mut elements);
                assert_eq!(elements, []);
            }

            #[test]
            fn sort_one_element_list() {
                let mut elements = [5];
                $sort_function(&mut elements);
                assert_eq!(elements, [5]);
            }

            #[test]
            fn sort_sorted_list() {
                let mut elements = [1, 2, 3, 4, 5];
                $sort_function(&mut elements);
                assert_eq!(elements, [1, 2, 3, 4, 5]);
            }

            #[test]
            fn sort_reverse_sorted_list() {
                let mut elements = [5, 4, 3, 2, 1];
                $sort_function(&mut elements);
                assert_eq!(elements, [1, 2, 3, 4, 5]);
            }

            #[test]
            fn sort_unsorted_list() {
                let mut elements = [3, 5, 1, 4, 2];
                $sort_function(&mut elements);
                assert_eq!(elements, [1, 2, 3, 4, 5]);
            }

            #[test]
            fn sort_list_with_duplicates() {
                let mut elements = [3, 5, 1, 4, 2, 3];
                $sort_function(&mut elements);
                assert_eq!(elements, [1, 2, 3, 3, 4, 5]);
            }
        }
    };
}

pub mod prelude {
    pub use super::bubble_sort::bubble_sort;
    pub use super::insertion_sort::insertion_sort;
    pub use super::shell_sort::shell_sort;
    pub use super::mergesort::merge_sort;
    pub use super::quicksort::quicksort;
}

pub mod bubble_sort;
pub mod insertion_sort;
pub mod shell_sort;
pub mod mergesort;
pub mod quicksort;
pub mod parallel;