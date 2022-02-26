use std::mem;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    // unsafe rust: https://doc.rust-lang.org/nightly/nomicon/
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn boxed(elem: T) -> Box<Self> {
        Box::new(Node { elem: elem, next: None })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    fn push(&mut self, elem: T) {
        let mut new_tail = Node::boxed(elem);
        let raw_tail: *mut _ = &mut *new_tail;
        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            // it's totally safe to play with pointers (r/write) in general.
            // problems arise when you try to dereference them => unsafe blocks!
            unsafe { (*self.tail).next = Some(new_tail); }
        }
        self.tail = raw_tail
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}

// $ rustup +nightly-2022-01-21 component add miri
// $ cargo +nightly-2022-01-21 miri test
// https://github.com/rust-lang/miri
// An experimental interpreter for Rust's mid-level intermediate representation
// (MIR). It can run binaries and test suites of cargo projects and detect
// certain classes of undefined behavior.
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_super_basic() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}

