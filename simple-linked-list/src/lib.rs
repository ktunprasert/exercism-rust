use std::iter::from_fn;

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct Node<T> {
    me: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(me: T, next: Option<Box<Node<T>>>) -> Self {
        Self { me, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // FILO
    pub fn push(&mut self, el: T) {
        self.size += 1;
        self.head = Some(Box::new(Node::new(el, self.head.take())))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.me
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.me)
    }

    #[must_use]
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        from_fn(|| self.pop()).fold(Self::new(), |mut ll, node| {
            ll.push(node);
            ll
        })
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |mut list, item| {
            list.push(item);
            list
        })
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = from_fn(|| linked_list.pop()).fold(Vec::new(), |mut vec, node| {
            vec.push(node);
            vec
        });

        vec.reverse();
        vec
    }
}
