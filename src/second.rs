
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        self.head = Link::Some(Box::new(Node {
            elem: elem,
            next: self.head.take(),
        }))
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn peek(&mut self) -> Option<&T> {
        // impl<T> Option<T> { pub fn as_ref(&self) -> Option<&T>; }
        self.head.as_ref().map(|node| { &node.elem })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| { &mut node.elem })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        let strval = "I am a string now";
        list.push(strval);

        print!("No peeking! {:#?}", list.peek());
        assert_eq!(list.peek(), Some(&strval));
        assert_eq!(list.pop(), Some(strval));
        assert_eq!(list.pop(), None);

        list.push(strval);
        list.peek_mut().map(|v| { *v = "Not anymore" });

        assert_eq!(list.peek(), Some(&"Not anymore"));
        assert_eq!(list.peek_mut(), Some(&mut "Not anymore"));
    }
}
