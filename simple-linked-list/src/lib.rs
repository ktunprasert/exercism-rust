#[derive(Default, Debug)]
pub struct SimpleLinkedList<T> {
    me: Option<T>,
    next: Option<Box<Self>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            me: None,
            next: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.me.is_none() && self.next.is_none()
    }

    pub fn len(&self) -> usize {
        if self.me.is_some() {
            1 + self.next.as_ref().map_or(0, |n| n.len())
        } else {
            0
        }
    }

    // FILO
    pub fn push(&mut self, element: T) {
        if self.is_empty() {
            self.me = Some(element);
        } else {
            let cpy = Box::new(Self {
                me: self.me.take(),
                next: self.next.take(),
            });

            self.me = Some(element);
            self.next = Some(cpy);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.next.is_none() {
            self.me.take()
        } else {
            let mut next = self.next.take().unwrap();
            let ret = self.me.take();
            self.me = next.me.take();
            self.next = next.next.take();

            ret
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.me.as_ref()
    }

    #[must_use]
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut reversed: Vec<T> = vec![];

        while let Some(next) = self.pop() {
            reversed.push(next);
        }

        Self::from_iter(reversed)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        let mut it = iter.into_iter();

        while let Some(item) = it.next() {
            list.push(item);
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec!();

        while let Some(el) = linked_list.pop() {
            vec.push(el);
        }

        vec.reverse();
        vec
    }
}
