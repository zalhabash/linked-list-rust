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
}
