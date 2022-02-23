use std::mem;

pub struct List {
    head: Link
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    value: i32,
    next: Link
}

impl List {
    fn new() -> Self {
        return List { head: Link::Empty };
    }

    fn push(&mut self, value: i32) {
        let node = Node {
            value: value,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }

        // Ugly way of doing this
        // let result;
        // match &mut self.head {
        //     Link::Empty => {
        //         result = Option::None;
        //     },
        //     Link::More(node) => {
        //         result = Option::Some(node.value);
        //         self.head = mem::replace(&mut node.next, Link::Empty);
        //     }
        // }
        // result
    }

    fn peek(&mut self) -> Option<i32> {
        // note the lack of semicolons (implicit return)
        match &self.head {
            Link::Empty => Option::None,
            Link::More(node) => Option::Some(node.value)
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(3));
        assert_eq!(list.peek(), Some(3));
        assert_eq!(list.pop(), Some(3));

        assert_eq!(list.peek(), Some(2));
        assert_eq!(list.peek(), Some(2));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
