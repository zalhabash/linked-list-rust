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

#[derive(Debug)]
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> IntoIter<T> {
    pub fn new(list: LinkedList<T>) -> Self {
        IntoIter { list }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.size, Some(self.list.size))
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
        let mut iterator_mut = IterMut::new(&mut list);

        *iterator_mut.next().unwrap() = 4;
        *iterator_mut.next().unwrap() = 5;
        *iterator_mut.next().unwrap() = 6;

        assert_eq!(iterator_mut.next(), None);
        assert_eq!(list, vec![4, 5, 6].into());
    }

    #[test]
    fn ownership_iterator_consumes_list() {
        let list: LinkedList<i32> = vec![1, 2, 3].into();
        let mut iterator_once = IntoIter::new(list);

        assert_eq!(iterator_once.next(), Some(1));
        assert_eq!(iterator_once.next(), Some(2));
        assert_eq!(iterator_once.next(), Some(3));
        assert_eq!(iterator_once.next(), None);
        assert_eq!(iterator_once.next(), None);
    }

    #[test]
    fn ownership_iterator_size_hint_returns_precise_size() {
        let list: LinkedList<i32> = vec![1, 2, 3].into();
        let mut iterator_once = IntoIter::new(list);

        assert_eq!(iterator_once.size_hint(), (3, Some(3)));

        iterator_once.next();
        assert_eq!(iterator_once.size_hint(), (2, Some(2)));
    }
}
