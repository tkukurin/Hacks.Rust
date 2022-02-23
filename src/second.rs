
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

/*
 * lifetimes could in principle be left out, but checking all the borrows would
 * be a huge whole-program analysis that would produce cryptic non-local errors.
 * Rust's system means all borrow checking can be done in each function body
 * independently => errors should be fairly local. Rust auto-infers:
 * 1 Only one reference in input, so the output must be derived from that input
 *   fn foo(&A) -> &B; // sugar for: fn foo<'a>(&'a A) -> &'a B;
 * 2 Many inputs, assume they're all independent
 *   fn foo(&A, &B, &C); // sugar for: fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);
 * 3 Methods, assume all output lifetimes are derived from `self`
 *   fn foo(&self, &B, &C) -> &D; // sugar for:
 *   fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;
 */
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // lifetime could be elided here according to rule 3 above
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    // elision applied here
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

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
