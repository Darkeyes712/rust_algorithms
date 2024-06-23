/// A linked list implementation in Rust.
#[derive(Clone, Debug)]
pub struct KolzoLinkedList<T> {
    /// The head of the linked list.
    head: Option<Box<Node<T>>>,
    /// The tail of the linked list.
    tail: Option<*mut Node<T>>, // Raw pointer for the tail
    /// The length of the linked list.
    length: u64,
}

/// A node in the linked list.
#[derive(Clone, Debug)]
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
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            data: value,
            next: None,
        }
    }
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
        Self {
            head: None,
            tail: None,
            length: 0,
        }
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
        let raw_node: *mut _ = &mut *new_node;

        match self.tail.take() {
            Some(tail) => unsafe {
                (*tail).next = Some(new_node);
                self.tail = Some(raw_node);
            },
            None => {
                self.head = Some(new_node);
                self.tail = Some(raw_node);
            }
        }

        self.length += 1;
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
        let mut current = self.head.as_ref().map(|node| &**node);
        while let Some(node) = current {
            print!("{:?} -> ", node.data);
            current = node.next.as_deref();
        }
        println!("None");
    }
}
