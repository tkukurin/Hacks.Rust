/*
 * Wrong implementation.
 *
 * Still a singly-linked list, but this time a queue.
 * Change semantics to append to head.next and keep a tail pointer.
 */

use std::mem;

pub struct List<'a, T> {
    head: Link<T>,
    // non-owning pointer = reference
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

// This won't work for some reason I need to investigate.
// #[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

// This won't work because T would need to implement the copy trait.
// impl<T> Clone for Node<T> {
//     fn clone(&self) -> Self {
//         Node { elem: self.elem, next: None }
//     }
// }

impl<T> Node<T> {
    fn boxed(elem: T) -> Box<Self> {
        Box::new(Node { elem: elem, next: None })
    }
}

impl<'a, T> List<'a, T> {
    fn new() -> Self {
        List { head: None, tail: None }
    }

    fn push(&'a mut self, elem: T) {
        // rust sin: storing a reference to yourself inside yourself
        let mut new_tail = Node::boxed(elem);
        let new_tail = match self.tail.take() {
            None => {
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            },
            Some(mut old) => {
                old.next = Some(new_tail);
                old.next.as_deref_mut()
            }
        };
        self.tail = new_tail;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            if self.head.is_none() {
                // if we forgot this, tail could become a dangling reference
                self.tail = None;
            }
            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_lol_this_works_pop() {
        let mut list = List::<i32>::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_lol_this_works_push() {
        let mut list = List::new();
        list.push(1);
    }

    #[test]
    fn test_fails_if_uncommented() {
        let mut list = List::new();
        // Using the list twice fails due to self-reference in push/pop:
        // "cannot borrow `list` as mutable more than once at a time"
        list.push(1);
        // assert_eq!(list.pop(), None);
    }
}

