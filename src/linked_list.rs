use std::mem::take;

#[derive(Debug)]
pub struct LinkedList {
    root: Option<Box<Node>>,
    size: usize,
}

#[derive(Debug)]
struct Node {
    next: Option<Box<Node>>,
    value: i32,
}

impl LinkedList {
    pub fn new() -> LinkedList {
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

    pub fn front(&self) -> Option<i32> {
        match &self.root {
            Some(first_node) => Some(first_node.value),
            None => None,
        }
    }

    pub fn push_front(&mut self, input: i32) {
        let old_root = take(&mut self.root);
        let new_node = Node {
            next: old_root,
            value: input,
        };
        self.root = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        let Some(popped_node) = take(&mut self.root) else {
            return None;
        };
        self.root = popped_node.next;
        self.size -= 1;
        Some(popped_node.value)
    }

    pub fn back(&self) -> Option<i32> {
        let Some(root_node) = self.root.as_ref() else {
            return None;
        };

        let mut last = root_node.as_ref();
        while let Some(next) = last.next.as_ref() {
            last = next.as_ref();
        }
        Some(last.value)
    }
}

impl From<Vec<i32>> for LinkedList {
    fn from(value: Vec<i32>) -> Self {
        let mut output = LinkedList::new();

        for item in value.into_iter().rev() {
            output.push_front(item);
        }

        output
    }
}

impl From<&[i32]> for LinkedList {
    fn from(value: &[i32]) -> Self {
        let mut output = LinkedList::new();

        for item in value.iter().cloned().rev() {
            output.push_front(item);
        }

        output
    }
}

impl Into<Vec<i32>> for LinkedList {
    fn into(mut self) -> Vec<i32> {
        let mut output = Vec::new();

        while let Some(item) = self.pop_front() {
            output.push(item);
        }

        output
    }
}

impl PartialEq for LinkedList {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.size == other.size
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.value == other.value
    }
}

#[cfg(test)]
mod test {
    use std::vec;

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
        assert_eq!(list.length(), 3)
    }

    #[test]
    fn is_empty_returns_true_for_a_fresh_empty_list() {
        let list = LinkedList::new();
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

        assert_eq!(list.front(), Some(2));
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
        let mut actual = LinkedList::new();
        assert_eq!(actual.pop_front(), None);
        assert_eq!(actual.length(), 0)
    }

    #[test]
    fn back_returns_an_option_of_the_last_value() {
        let input = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(input.back(), Some(3));
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
    fn identical_nodes_are_equal_to_each_other() {
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

        let c = Node {
            next: None,
            value: 5,
        };
        let d = Node {
            next: Some(Box::new(Node {
                next: None,
                value: 5,
            })),
            value: 5,
        };

        assert_ne!(c, d);
    }

    #[test]
    fn construct_linked_list_from_vector() {
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
        assert_eq!(expected, actual);
    }

    #[test]
    fn construct_linked_list_from_vector_without_taking_ownership() {
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
        assert_eq!(expected, actual);
    }

    #[test]
    fn construct_vector_from_linked_list() {
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
        assert_eq!(expected, actual);
    }
}
