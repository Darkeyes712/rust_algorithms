/// A node in the linked list.
#[derive(Debug, Clone, PartialEq)]
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

impl<T: std::fmt::Debug + Clone> KolzoLinkedList<T> {
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

    /// Adds a value to the beginning of the linked list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = KolzoLinkedList::new();
    /// list.prepend(1);
    /// list.prepend(2);
    /// list.prepend(3);
    /// assert_eq!(list.length, 3);
    /// // The list now looks like: 3 -> 2 -> 1 -> None
    /// ```
    pub fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        let new_node_raw_pointer: &mut _ = &mut *new_node;

        if self.head.is_none() {
            self.tail = Some(new_node_raw_pointer);
        }

        new_node.next = self.head.take();
        self.head = Some(new_node);

        self.length += 1;
    }

    /// Removes the first element from the linked list and returns it, if it exists.
    ///
    /// # Returns
    ///
    /// - `Some(T)` containing the value of the first element if the list is not empty.
    /// - `None` if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(1);
    /// list.append(2);
    ///
    /// assert_eq!(list.pop_first(), Some(1));
    /// assert_eq!(list.pop_first(), Some(2));
    /// assert_eq!(list.pop_first(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn pop_first(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut old_head = self.head.take().unwrap();
        self.head = old_head.next.take();

        if self.head.is_none() {
            self.tail = None;
        }

        Some(old_head.data)
    }

    /// Retrieves a reference to the element at the specified index in the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve. Must be a non-negative integer.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` containing a reference to the element at the specified index if it exists.
    /// * `None` if the index is out of bounds or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(10);
    /// list.append(20);
    /// list.append(30);
    ///
    /// assert_eq!(list.get(0), Some(&10));
    /// assert_eq!(list.get(1), Some(&20));
    /// assert_eq!(list.get(2), Some(&30));
    /// assert_eq!(list.get(3), None);
    /// assert_eq!(list.get(-1), None);
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn get(&self, index: i64) -> Option<&T> {
        if index.is_negative() || index as u64 >= self.length {
            return None;
        }

        let mut head_node = &self.head;
        let mut count = 0;
        while let Some(ref node) = head_node {
            if count == index {
                return Some(&node.data);
            }
            head_node = &node.next;
            count += 1;
        }

        None
    }

    /// Updates the value of the element at the specified index in the linked list
    /// and returns the old value.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to update. Must be a non-negative integer.
    /// * `value` - The new value to set at the specified index.
    ///
    /// # Returns
    ///
    /// * `Some(T)` containing the old value of the element at the specified index if it exists.
    /// * `None` if the index is out of bounds or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(10);
    /// list.append(20);
    /// list.append(30);
    ///
    /// assert_eq!(list.set(1, 25), Some(20)); // Replaces the value at index 1
    /// assert_eq!(list.get(1), Some(&25));    // Verifies the new value at index 1
    /// assert_eq!(list.set(3, 40), None);     // Index out of bounds
    /// assert_eq!(list.set(-1, 50), None);    // Negative index
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn set(&mut self, index: i64, value: T) -> Option<T> {
        if index.is_negative() || index as u64 >= self.length {
            return None;
        }

        let mut head_node = &mut self.head;
        let mut count = 0;

        while let Some(ref mut node) = head_node {
            if count == index {
                let old_val = std::mem::replace(&mut node.data, value);
                return Some(old_val);
            }
            head_node = &mut node.next;
            count += 1;
        }
        None
    }

    pub fn playground(&self) {
        let mut new_ll: KolzoLinkedList<i32> = KolzoLinkedList::new();

        new_ll.append(2);
        new_ll.append(3);
        new_ll.append(4);

        let test_head = new_ll.head;
        let test_tail = new_ll.tail;
        let test_length = new_ll.length;

        match test_head {
            Some(head) => {
                println!("HEAD DATA {:?}", head.data);
                println!("HEAD NEXT {:?}", head.next);
            }
            None => (),
        }

        match test_tail {
            Some(tail) => unsafe {
                println!("TAIL DATA {:?}", (*tail).data);
                println!("TAIL NEXT {:?}", (*tail).next);
            },
            None => (),
        }

        println!("LENGHT {}", test_length);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_pop() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.length, 3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.length, 2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.length, 1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.length, 0);

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_prepend() {
        let mut list = KolzoLinkedList::new();

        list.prepend(1);
        list.prepend(2);
        list.prepend(3);

        assert_eq!(list.length, 3);

        let mut current = list.head.as_ref();
        assert_eq!(current.map(|node| &node.data), Some(&3));
        current = current.unwrap().next.as_ref();
        assert_eq!(current.map(|node| &node.data), Some(&2));
        current = current.unwrap().next.as_ref();
        assert_eq!(current.map(|node| &node.data), Some(&1));
        current = current.unwrap().next.as_ref();
        assert_eq!(current, None);
    }

    #[test]
    fn test_pop_first() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        assert_eq!(list.pop_first(), None);

        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.pop_first(), Some(1));
        assert_eq!(list.pop_first(), Some(2));
        assert_eq!(list.pop_first(), Some(3));

        assert_eq!(list.pop_first(), None);
    }

    #[test]
    fn test_get() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), None);
        assert_eq!(list.get(-1), None);

        list.append(10);
        list.append(20);
        list.append(30);

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&30));

        assert_eq!(list.get(3), None);

        assert_eq!(list.get(-1), None);
    }

    #[test]
    fn test_set() {
        // Create a new empty linked list
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        assert_eq!(list.set(0, 10), None);
        assert_eq!(list.set(1, 20), None);
        assert_eq!(list.set(-1, 30), None);

        list.append(10);
        list.append(20);
        list.append(30);

        assert_eq!(list.set(0, 15), Some(10));
        assert_eq!(list.set(1, 25), Some(20));
        assert_eq!(list.set(2, 35), Some(30));

        assert_eq!(list.get(0), Some(&15));
        assert_eq!(list.get(1), Some(&25));
        assert_eq!(list.get(2), Some(&35));

        assert_eq!(list.set(3, 40), None);

        assert_eq!(list.set(-1, 50), None);
    }
}
