// like Box, but shared
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone()
        })) }
    }

    pub fn tail(&self) -> List<T> {
        // `and_then` == flatmap. `x.and_then(f1).and_then(f2)` short for:
        // match x {
        //   None => None,
        //   val => match val.map(f1) {
        //     None => None,
        //     val => val.map(f2),
        //   }
        // }
        // (note that n.next == Option<T>)
        List { head: self.head.as_ref().and_then(|n| n.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let l1 = List::new();
        assert_eq!(l1.head(), None);

        let l2 = l1.prepend(1).prepend(2).prepend(3);
        assert_eq!(l1.head(), None);
        assert_eq!(l2.head(), Some(&3));

        assert_eq!(l2.tail().head(), Some(&2));
    }
}
