
use std::mem;

pub struct List {
    head: Link
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: i32) {
        self.head = Link::Some(Box::new(Node {
            elem: elem,
            next: self.head.take(),
        }))
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}
