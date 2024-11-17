use super::*;

#[derive(Debug)]
pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Iter {
            node: list.root.as_ref().map(|node| node.as_ref()),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.node else {
            return None;
        };

        self.node = node.next.as_ref().map(|next_node| next_node.as_ref());
        Some(&node.value)
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    node: Option<&'a mut Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(list: &'a mut LinkedList<T>) -> Self {
        IterMut {
            node: list.root.as_mut().map(|node| node.as_mut()),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.node.take() else {
            return None;
        };

        self.node = node.next.as_mut().map(|next_node| next_node.as_mut());
        Some(&mut node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowing_iterator_returns_next() {
        let list: LinkedList<i32> = vec![1, 2, 3].into();
        let mut iterator = Iter::new(&list);

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn mutably_borrowing_iterator_allows_mutation_of_values() {
        let mut list: LinkedList<i32> = vec![1, 2, 3].into();
        let mut mut_iterator = IterMut::new(&mut list);

        *mut_iterator.next().unwrap() = 4;
        *mut_iterator.next().unwrap() = 5;
        *mut_iterator.next().unwrap() = 6;

        assert_eq!(mut_iterator.next(), None);
        assert_eq!(list, vec![4, 5, 6].into());
    }
}
