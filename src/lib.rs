pub struct LinkedList<T> {
    inner: Option<Box<Node<T>>>,
    count: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            inner: None,
            count: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let head = Node {
            item,
            next: self.inner.take(),
        };
        self.inner = Some(Box::new(head));
        self.count += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.as_ref()?.get_nth(index)
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn get_nth(&self, index: usize) -> Option<&T> {
        match index {
            0 => Some(&self.item),
            i => match &self.next {
                None => None,
                Some(next) => Self::get_nth(next, i - 1),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn empty_list() {
        let list: LinkedList<u32> = LinkedList::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.get(0), None);
    }

    #[test]
    fn one_item() {
        let mut list: LinkedList<u32> = LinkedList::new();
        list.push_front(42);

        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(&42));
    }

    #[test]
    fn two_items() {
        let mut list: LinkedList<u32> = LinkedList::new();
        list.push_front(42);
        list.push_front(37);

        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(&37));
        assert_eq!(list.get(1), Some(&42));
    }
}
