macro_rules! list_tests {
    ($list_maker:expr) => {
        #[cfg(tests)]
        mod tests {
            use super::*;
            #[test]
            fn test_push_pop() {
                let mut list = $list_maker();

                list.push_front(1);
                list.push_front(2);
                list.push_front(3);

                assert_eq!(list.pop_front(), Some(3));
                assert_eq!(list.pop_front(), Some(2));
                assert_eq!(list.pop_front(), Some(1));
                assert_eq!(list.pop_front(), None);

                list.push_back(4);
                list.push_back(5);
                list.push_back(6);

                assert_eq!(list.pop_back(), Some(6));
                assert_eq!(list.pop_back(), Some(5));
                assert_eq!(list.pop_back(), Some(4));
                assert_eq!(list.pop_back(), None);
            }

            #[test]
            fn test_len_is_empty() {
                let mut list = $list_maker();

                assert_eq!(list.len(), 0);
                assert!(list.is_empty());

                list.push_front(1);
                list.push_front(2);
                list.push_front(3);

                assert_eq!(list.len(), 3);
                assert!(!list.is_empty());

                list.pop_front();
                list.pop_front();

                assert_eq!(list.len(), 1);
                assert!(!list.is_empty());

                list.pop_front();

                assert_eq!(list.len(), 0);
                assert!(list.is_empty());
            }

            #[test]
            fn test_index() {
                let mut list = $list_maker();

                list.push_back(1);
                list.push_back(2);
                list.push_back(3);

                assert_eq!(list[0], 1);
                assert_eq!(list[1], 2);
                assert_eq!(list[2], 3);
            }

            #[test]
            fn test_iterator() {
                let mut list = $list_maker();

                list.push_back(1);
                list.push_back(2);
                list.push_back(3);

                let mut iter = list.iter();
                assert_eq!(iter.next(), Some(&1));
                assert_eq!(iter.next(), Some(&2));
                assert_eq!(iter.next(), Some(&3));
                assert_eq!(iter.next(), None);
            }
        }
    };
}

pub mod list;
pub mod linked_list;
pub mod array_list;
pub mod queue;
pub mod stack;