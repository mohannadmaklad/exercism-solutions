use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = &self.head;

        while node.is_some() {
            len = len + 1;
            node = &node.as_ref().unwrap().next;
        }

        len as usize
    }

    pub fn push(&mut self, element: T) {
        let new_node: Node<T> = Node {
            data: element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut element = self.head.take();
        self.head = match element {
            Some(ref mut boxed_node) => boxed_node.next.take(),
            None => None,
        };
        element.map(|boxed_data| boxed_data.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_data| &boxed_data.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList { head: None };
        let mut node = &self.head;
        while node.is_some() {
            list.push(node.as_ref().unwrap().data.clone());
            node = &node.as_ref().unwrap().next;
        }
        list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList { head: None };
        for i in iter {
            list.push(i.clone());
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list: Vec<T> = vec![];
        let mut node = linked_list.pop();
        while node.is_some() {
            list.insert(0, node.unwrap());
            node = linked_list.pop();
        }
        list
    }
}
