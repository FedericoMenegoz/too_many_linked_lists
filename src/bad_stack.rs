use std::mem;

#[derive(PartialEq, Debug)]
pub struct List<T> {
    head: Link<T>,
}
#[derive(PartialEq, Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

/* null pointer optimization:
    -   This enum will have the same size as a Box<Node>. Because Box is a pointer and pointers are always non-zero,
        the compiler assosiate to the empty variant the all zero bit instead of having extra space to store the tag
        for the variant check code in null_optimization
*/
#[derive(PartialEq, Debug)]
enum Link<T> {
    More(Box<Node<T>>),
    Empty,
}

impl<T> List<T> {
    /// Create a new list.
    pub fn new() -> Self {
        List {
            head: Link::Empty::<T>,
        }
    }

    /// Push a new node at the front of the list.
    pub fn push(&mut self, val: T) {
        let new_node = Node {
            val,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }
    // -----------------------------------------
    // POP VERSION WITHOUT THE pop_node() METHOD
    // -----------------------------------------
    // pub fn pop(&mut self) -> Option<T> {
    //     let node_to_delete = mem::replace(&mut self.head, Link::Empty);

    //     match node_to_delete {
    //         Link::More(l) => {
    //             self.head = l.next;
    //             Some(l.val)
    //         }
    //         Link::Empty => None,
    //     }
    // }
    // -----------------------------------------
    //        OPTIMIZED POP VERSION
    // -----------------------------------------
    pub fn pop (&mut self) -> Option<T> {
        match self.pop_node() {
            Link::More(l) => Some(l.val),
            Link::Empty => None
        }
    }
    // Internal function to benefit both pop() and drop() [Bonus Section for Premature Optimization]
    fn pop_node(&mut self) -> Link<T> {

        let node_to_delete = mem::replace(&mut self.head, Link::Empty);
        
        match node_to_delete {
            Link::More(mut l) => { 
                mem::swap(&mut self.head, &mut l.next);
                Link::More(l)
            },
            Link::Empty => Link::Empty
        }
    }
}


impl<T> Drop for List<T> {

    // Before deallocating, the head needs to be fully dropped, which means that every node needs to be dropped,
    // each of which will be deallocated only after the drop finishes. This may lead to pushing many drops onto 
    // the stack, potentially causing a stack overflow.
    // Implementing the trait ourselves can help avoid this problem: iterating through the list while there
    // are some nodes and dropping each node in every cycle.
    // ------------------------------
    // WITHOUT THE drop_node() METHOD
    // ------------------------------
    // fn drop(&mut self) {
    //     let mut current_node = mem::replace(&mut self.head, Link::Empty);

    //     while let Link::More(mut some_node) = current_node {
    //         current_node = mem::replace(&mut some_node.next, Link::Empty)
    //         // some_node will be dropped here
    //     }
    // }

    fn drop(&mut self) {
        while let Link::More(_some_node) = self.pop_node() { }
    }
}

#[cfg(test)]
mod test {
    use crate::bad_stack::{Link, List};

    #[test]
    fn new_create_an_empty_list() {
        assert_eq!(
            List::<i32>::new(),
            List {
                head: Link::Empty::<i32>
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
}
