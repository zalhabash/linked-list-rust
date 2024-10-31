#[derive(Debug)]
pub struct LinkedList<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            root: None,
            size: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn front(&self) -> Option<&T> {
        match &self.root {
            Some(first_node) => Some(&first_node.value),
            None => None,
        }
    }

    pub fn push_front(&mut self, input: T) {
        let old_root = self.root.take();
        let new_node = Node {
            next: old_root,
            value: input,
        };

        self.root = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let Some(popped_node) = self.root.take() else {
            return None;
        };

        self.root = popped_node.next;
        self.size -= 1;
        Some(popped_node.value)
    }

    pub fn back(&self) -> Option<&T> {
        let Some(root_node) = self.root.as_ref() else {
            return None;
        };

        let mut last = root_node.as_ref();
        while let Some(next) = last.next.as_ref() {
            last = next.as_ref();
        }
        Some(&last.value)
    }

    pub fn push_back(&mut self, input: T) {
        let new_node = Some(Box::new(Node {
            next: None,
            value: input,
        }));

        let Some(root_node) = self.root.as_mut() else {
            self.root = new_node;
            self.size += 1;
            return;
        };

        let mut last = root_node.as_mut();
        while last.next.is_some() {
            last = last.next.as_mut().unwrap();
        }
        last.next = new_node;
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut last = &mut self.root;
        while last.as_ref().is_some_and(|last| last.next.is_some()) {
            last = &mut last.as_mut().unwrap().next;
        }
        let Some(last) = last.take() else {
            return None;
        };
        self.size -= 1;
        return Some(last.value);
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut output = LinkedList::new();

        for item in value.into_iter().rev() {
            output.push_front(item);
        }

        output
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(value: &[T]) -> Self {
        let mut output = LinkedList::new();

        for item in value.iter().cloned().rev() {
            output.push_front(item);
        }

        output
    }
}

impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut output = Vec::new();

        while let Some(item) = self.pop_front() {
            output.push(item);
        }

        output
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.size == other.size
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.value == other.value
    }
}

impl<T: Eq> Eq for Node<T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn length_returns_number_of_nodes_in_linked_list() {
        let mut list = LinkedList::new();
        assert_eq!(list.length(), 0);

        list.push_front(6);
        assert_eq!(list.length(), 1);

        list.push_front(5);
        assert_eq!(list.length(), 2);

        list.push_front(4);
        assert_eq!(list.length(), 3);

        let mut list = LinkedList::new();
        assert_eq!(list.length(), 0);

        list.push_front("a".to_string());
        assert_eq!(list.length(), 1);

        list.push_front("b".to_string());
        assert_eq!(list.length(), 2);

        list.push_front("c".to_string());
        assert_eq!(list.length(), 3);

        let mut list = LinkedList::new();
        assert_eq!(list.length(), 0);

        list.push_front(1.1);
        assert_eq!(list.length(), 1);

        list.push_front(2.2);
        assert_eq!(list.length(), 2);

        list.push_front(3.3);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn is_empty_returns_true_for_a_fresh_empty_list() {
        let list = LinkedList::<i32>::new();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn is_empty_returns_true_for_an_empty_list_after_pushing_and_popping_everything() {
        let mut list = LinkedList::new();
        list.push_front(5);
        list.pop_front();

        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn is_empty_returns_false_for_a_non_empty_list() {
        let mut list = LinkedList::new();
        list.push_front(5);

        assert_eq!(list.is_empty(), false);
    }

    #[test]
    fn front_peeks_at_the_first_value_in_list() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);

        assert_eq!(list.front(), Some(&2));
    }

    #[test]
    fn push_front_once_adds_one_node() {
        let mut actual = LinkedList::new();
        actual.push_front(5);
        assert_eq!(
            actual,
            LinkedList {
                root: Some(Box::new(Node {
                    next: None,
                    value: 5
                })),
                size: 1,
            }
        )
    }

    #[test]
    fn push_front_twice_adds_two_nodes() {
        let mut actual = LinkedList::new();
        actual.push_front(5);
        actual.push_front(6);

        let a = Box::new(Node {
            next: None,
            value: 5,
        });
        let b = Box::new(Node {
            next: Some(a),
            value: 6,
        });

        assert_eq!(
            actual,
            LinkedList {
                root: Some(b),
                size: 2,
            }
        )
    }

    #[test]
    fn pop_front_on_list_with_one_node_makes_the_list_empty() {
        let mut actual = LinkedList {
            root: Some(Box::new(Node {
                next: None,
                value: 5,
            })),
            size: 1,
        };
        assert_eq!(actual.pop_front(), Some(5));
        assert_eq!(actual, LinkedList::new())
    }

    #[test]
    fn pop_front_on_empty_list_pops_nothing() {
        let mut actual = LinkedList::<i32>::new();
        assert_eq!(actual.pop_front(), None);
        assert_eq!(actual.length(), 0)
    }

    #[test]
    fn back_returns_an_option_of_the_last_value() {
        let input = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(input.back(), Some(&3));
    }

    #[test]
    fn push_back_adds_a_node_to_the_end_of_linked_list() {
        let mut actual = LinkedList::new();
        actual.push_back(1);
        assert_eq!(actual, LinkedList::from(vec![1]));
    }

    #[test]
    fn push_back_adds_several_nodes_to_the_end_of_linked_list() {
        let mut actual = LinkedList::new();
        actual.push_back(1);
        actual.push_back(2);
        actual.push_back(3);
        assert_eq!(actual, LinkedList::from(vec![1, 2, 3]));
    }

    #[test]
    fn pop_back_returns_nothing_for_empty_linked_list() {
        let mut actual = LinkedList::<i32>::new();
        assert_eq!(actual.pop_back(), None);
        assert_eq!(actual, LinkedList::new());
    }

    #[test]
    fn pop_back_returns_last_element() {
        let mut actual = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(actual.pop_back(), Some(3));
        assert_eq!(actual, LinkedList::from(vec![1, 2]));
    }

    #[test]
    fn linked_list_works_as_stack() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(9);
        list.push_front(8);
        list.push_front(7);
        assert_eq!(list.pop_front(), Some(7));
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(8));
        assert_eq!(list.pop_front(), Some(9));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list, LinkedList::new());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn identical_nodes_are_partially_equal_to_each_other() {
        let a = Node {
            next: None,
            value: 5,
        };
        let b = Node {
            next: None,
            value: 5,
        };

        assert_eq!(a, a);
        assert_eq!(a, b);
    }

    #[test]
    fn different_nodes_are_not_equal_to_each_other() {
        let a = Node {
            next: None,
            value: 5,
        };
        let b = Node {
            next: None,
            value: 6,
        };

        assert_ne!(a, b);

        let a = Node {
            next: None,
            value: 5,
        };
        let b = Node {
            next: Some(Box::new(Node {
                next: None,
                value: 5,
            })),
            value: 5,
        };

        assert_ne!(a, b);
    }

    #[test]
    fn construct_linked_list_from_vector_of_i32() {
        let input: Vec<i32> = vec![1, 2, 3];
        let actual = LinkedList::from(input);
        // let actual = input.into();
        let expected = LinkedList {
            root: Some(Box::new(Node {
                value: 1,
                next: Some(Box::new(Node {
                    value: 2,
                    next: Some(Box::new(Node {
                        value: 3,
                        next: None,
                    })),
                })),
            })),
            size: 3,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn construct_linked_list_from_vector_of_i32_without_taking_ownership() {
        let input = [1, 2, 3];
        let actual = LinkedList::from(input.as_slice());
        // let actual = input.into();
        let expected = LinkedList {
            root: Some(Box::new(Node {
                value: 1,
                next: Some(Box::new(Node {
                    value: 2,
                    next: Some(Box::new(Node {
                        value: 3,
                        next: None,
                    })),
                })),
            })),
            size: 3,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn construct_linked_list_from_vector_of_string() {
        let input = vec!["a".to_string(), "b".to_string()];
        let actual = LinkedList::from(input);
        let expected = LinkedList {
            root: Some(Box::new(Node {
                value: "a".to_string(),
                next: Some(Box::new(Node {
                    value: "b".to_string(),
                    next: None,
                })),
            })),
            size: 2,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn construct_linked_list_from_vector_of_floats() {
        let input = vec![1.0, 2.0];
        let actual = LinkedList::from(input);
        let expected = LinkedList {
            root: Some(Box::new(Node {
                value: 1.0,
                next: Some(Box::new(Node {
                    value: 2.0,
                    next: None,
                })),
            })),
            size: 2,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn assert_a_linked_list_containing_nan_is_not_equal_to_itself() {
        let input = vec![f64::NAN];
        let list = LinkedList::from(input);

        assert_ne!(list, list);
    }

    #[test]
    fn construct_vector_from_linked_list_of_i32() {
        let input = LinkedList {
            root: Some(Box::new(Node {
                value: 1,
                next: Some(Box::new(Node {
                    value: 2,
                    next: Some(Box::new(Node {
                        value: 3,
                        next: None,
                    })),
                })),
            })),
            size: 3,
        };
        let expected = vec![1, 2, 3];
        let actual: Vec<i32> = input.into();
        assert_eq!(actual, expected);
    }
}
