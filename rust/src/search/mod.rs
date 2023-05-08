macro_rules! generate_search_tests {
    ($search_function:path) => {
        mod tests {
            use super::*;
            #[test]
            fn test_empty() {
                let empty: [i32; 0] = [];
                assert_eq!($search_function(&empty, &1), None);
            }

            #[test]
            fn test_single() {
                let single = [1];
                assert_eq!($search_function(&single, &1), Some(0));
                assert_eq!($search_function(&single, &2), None);
            }

            #[test]
            fn test_multiple() {
                let multiple = [1, 2, 3];
                assert_eq!($search_function(&multiple, &1), Some(0));
                assert_eq!($search_function(&multiple, &2), Some(1));
                assert_eq!($search_function(&multiple, &3), Some(2));
                assert_eq!($search_function(&multiple, &4), None);
            }

            #[test]
            fn test_duplicates() {
                let duplicates = [1, 2, 2, 3];
                assert_eq!($search_function(&duplicates, &1), Some(0));
                assert_eq!($search_function(&duplicates, &2), Some(1));
                assert_eq!($search_function(&duplicates, &3), Some(3));
                assert_eq!($search_function(&duplicates, &4), None);
            }

            #[test]
            fn test_non_existing() {
                let duplicates = [0, 2, 2, 3];
                assert_eq!($search_function(&duplicates, &1), None);
            }
        }
    };
}

pub mod linear_search;
pub mod binary_search;
pub mod interpolation_search;