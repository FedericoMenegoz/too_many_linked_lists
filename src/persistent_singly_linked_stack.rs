use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Create an empty list.
    pub fn new() -> Self {
        List { head: None }
    }

    /// Takes a list and a val and return a new list with the new val in front.
    pub fn prepend(&self, val: T) -> List<T> {
        let new_node = Node {
            val,
            next: self.head.clone(),
        };
        List {
            head: Some(Rc::new(new_node)),
        }
    }

    /// Return a reference to the tail of the list.
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// Return a reference to the value in front of the list.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    /// Return an Iterator of immutable reference
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}
#[cfg(test)]
mod test {
    use crate::persistent_singly_linked_stack::List;

    #[test]
    fn basics() {
        let list = List::<i32>::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.prepend(4);
        assert_eq!(list.tail().head(), Some(&3));

        let list = List::<i32>::new();
        assert_eq!(list.tail().head(), None);
    }

    #[test]
    fn iter_list() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
