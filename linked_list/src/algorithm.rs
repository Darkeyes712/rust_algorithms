/// A node in the linked list.
#[derive(Debug)]
pub struct Node<T> {
    /// The data stored in the node.
    data: T,
    /// The next node in the linked list.
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Creates a new node with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the node.
    ///
    /// # Examples
    ///
    /// ```
    /// let node = Node::new(5);
    /// assert_eq!(node.data, 5);
    /// assert!(node.next.is_none());
    /// ```
    pub fn new(value: T) -> Self {
        Node {
            data: value,
            next: None,
        }
    }
}

/// A singly linked list implementation in Rust.
#[derive(Debug)]
pub struct KolzoLinkedList<T> {
    /// The head of the linked list.
    head: Option<Box<Node<T>>>,
    /// The tail of the linked list, represented as a raw pointer for efficient appending.
    tail: Option<*mut Node<T>>,
    /// The length of the linked list.
    length: u64,
}

impl<T: std::fmt::Debug> KolzoLinkedList<T> {
    /// Creates a new empty linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// let list: KolzoLinkedList<i32> = KolzoLinkedList::new();
    /// assert_eq!(list.length, 0);
    /// ```
    pub fn new() -> Self {
        KolzoLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Prints the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoLinkedList::new();
    /// list.append(1);
    /// list.append(2);
    /// list.append(3);
    /// list.print(); // Output: 1 -> 2 -> 3 -> None
    /// ```
    pub fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{:?} -> ", node.data);
            current = node.next.as_ref();
        }
        println!("None");
    }

    /// Appends a value to the end of the linked list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoLinkedList::new();
    /// list.append(1);
    /// list.append(2);
    /// list.append(3);
    /// assert_eq!(list.length, 3);
    /// ```
    pub fn append(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        let new_node_pointer: *mut _ = &mut *new_node;

        match self.tail {
            Some(tail_pointer) => unsafe {
                (*tail_pointer).next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }

        self.tail = Some(new_node_pointer);
        self.length += 1;
    }

    /// Removes and returns the last element from the linked list.
    ///
    /// # Returns
    ///
    /// * `Option<T>` - The value of the removed node if the list is not empty, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoLinkedList::new();
    /// list.append(1);
    /// list.append(2);
    /// list.append(3);
    ///
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            let head = self.head.take().unwrap();
            self.tail = None;
            self.length -= 1;

            return Some(head.data);
        }

        let mut current = self.head.as_mut().map(|node| &mut **node);
        while let Some(node) = current {
            if node.next.as_ref().unwrap().next.is_none() {
                let tail = node.next.take().unwrap();
                self.tail = Some(node as *mut Node<T>);
                self.length -= 1;
                return Some(tail.data);
            }
            current = node.next.as_mut().map(|node| &mut **node);
        }

        None
    }
}
