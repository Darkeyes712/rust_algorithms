/// A node in the doubly linked list.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    /// The data stored in the node.
    data: T,
    /// The previous node in the doubly linked list.
    previous: Option<Box<Node<T>>>,
    /// The next node in the doubly linked list.
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            data: value,
            previous: None,
            next: None,
        }
    }
}

/// A doubly linked list implementation in Rust.
#[derive(Debug)]
pub struct KolzoDoublyLinkedList<T> {
    /// The head of the doubly linked list.
    head: Option<Box<Node<T>>>,
    /// The tail of the doubly linked list, represented as a raw pointer for efficient appending.
    tail: Option<*mut Node<T>>,
    /// The length of the doubly linked list.
    length: u64,
}

impl<T: std::fmt::Debug + Clone> KolzoDoublyLinkedList<T> {
    /// Creates a new empty doubly linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// let list: KolzoDoublyLinkedList<i32> = KolzoDoublyLinkedList::new();
    /// assert_eq!(list.length, 0);
    /// ```
    pub fn new() -> Self {
        KolzoDoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Prints the doubly linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoDoublyLinkedList::new();
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
    /// Appends a new node with the given value to the end of the doubly linked list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be stored in the new node.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoDoublyLinkedList::new();
    /// list.append(1);
    /// list.append(2);
    /// list.append(3);
    /// assert_eq!(list.length, 3);
    /// ```
    ///
    /// # Safety
    ///
    /// This method uses raw pointers to modify the internal structure of the doubly linked list.
    /// It is marked as `unsafe` because dereferencing raw pointers can lead to undefined behavior
    /// if not done correctly.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let new_node_ptr: *mut _ = Box::into_raw(new_node);

        unsafe {
            if self.head.is_none() {
                self.head = Some(Box::from_raw(new_node_ptr));
                self.tail = Some(new_node_ptr);
            } else {
                if let Some(current) = self.tail {
                    (*current).next = Some(Box::from_raw(new_node_ptr));
                    (*new_node_ptr).previous = Some(Box::from_raw(current));
                    self.tail = Some(new_node_ptr);
                }
            }

            self.length += 1;
        }
    }

    pub fn pop(&mut self, value: T) {
        // Some code
    }

    pub fn prepend(&mut self, value: T) {
        // Some code
    }

    pub fn pop_first(&mut self, value: T) {
        // Some code
    }

    pub fn get(&mut self, value: T) {
        // Some code
    }

    pub fn set(&mut self, value: T) {
        // Some code
    }

    pub fn insert(&mut self, value: T) {
        // Some code
    }

    pub fn remove(&mut self, value: T) {
        // Some code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
