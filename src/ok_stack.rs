#[derive(PartialEq, Debug)]
pub struct List<T> {
    head: Link<T>,
}
#[derive(PartialEq, Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;


pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl<T> List<T> {
    /// Create a new list.
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    /// Push a new node at the front of the list.
    pub fn push(&mut self, val: T) {
        let new_node = Node {
            val,
            // mem::replace(&mut self.head, None) == self.head.take()
            next: self.head.take(),
        };

        self.head = Link::Some(Box::new(new_node));
    }

    /// Pop the node in front of the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    /// Return an immutable reference to the node in front of the list, if any.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    /// Return a mutable reference to the node in front of the list, if any.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self).into_iter()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }

}


impl<T> Drop for List<T> {
    
    fn drop(&mut self) {
        let mut current_node = self.head.take();

        while let Some(mut some_node) = current_node {
            current_node = some_node.next.take();
            // some_node will be dropped here
        }
    }
}

// This is for into_iter()
impl <T> Iterator for IntoIter<T> {

    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl <'a, T> Iterator for Iter<'a, T>  {
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
    use crate::ok_stack::{List};

    #[test]
    fn new_create_an_empty_list() {
        assert_eq!(
            List::<i32>::new(),
            List {
                head: None
            }
        );
    }

    #[test]
    fn pop_an_empty_list_should_return_none() {
        assert_eq!(List::<i32>::new().pop(), None);
    }

    #[test]
    fn push_and_pop_one_val() {
        let mut l = List::new();

        l.push(1);

        assert_eq!(l.pop(), Some(1));
    }

    #[test]
    fn push_and_pop_many() {
        let mut l = List::new();

        l.push(1);
        l.push(2);
        l.push(3);

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
    #[test]
    fn peek_ref_empty_list () {
        let l = List::<i32>::new();
        assert_eq!(l.peek(), None)
    }
    #[test]
    fn peek_ref () {
        let mut l = List::new();

        l.push(1);
        assert_eq!(l.peek(), Some(&1))
    }
    #[test]
    fn peek_mut_ref_empty_list () {
        let mut l = List::<i32>::new();
        assert_eq!(l.peek_mut(), None)
    }

    #[test]
    fn peek_mut_ref () {
        let mut l = List::new();

        l.push(1);
        l.peek_mut().map(|value| {
            *value = 2
        });
        assert_eq!(l.peek_mut(), Some(&mut 2))
    }

    #[test]
    fn list_into_iter() {
        let mut list = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.into_iter().collect::<Vec<i32>>(), vec![3,2,1]);
    }

    #[test]
    fn list_iter() {
        let mut list = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
