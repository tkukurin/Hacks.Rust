
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

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// Tuple: intoiter.0 == List<T>
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // pre-rust 1.4 this was `.as_ref().map(|n| &**n)`
            // or "turbofish" `.as_ref().map::<&Node<T>, _>(|n| &n)`
            // because `map<U, F>(self, f: F) -> Option<U>`
            // and it lets the compiler know to apply coercion to `&**n`
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

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
