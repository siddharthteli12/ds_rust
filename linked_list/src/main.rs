use std::env::consts;

struct LinkedList<T> {
    head: Option<*const Node<T>>,
    tail: Option<*const Node<T>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

struct Node<T> {
    value: T,
    left: Option<*const Node<T>>,
    right: Option<*const Node<T>>,
}

fn main() {
    // Features to added
    /*
    Create a new linked list.
    Insertion & Deletetion from link list
    Indexing elements.
    Impl iterator trait for link list.
    Length & is empty check.
    Search element & return index.
    Slice link list, return sub list from given index.
    Impl partialEq & Eq for comparsion.
    Convert from vec to linked list or vice versa.
    Display & Debug trait for linked list.
    Error handling for link list.
    Add test cases.
    Append link list to link list.
    Append value to link list.
    Add value in front of link list.
    Remove any value from linked list.
    Find value in link list.
     */
}
