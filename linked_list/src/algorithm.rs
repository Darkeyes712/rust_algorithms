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

        if let Some(node) = &self.head {
            if node.next.is_none() {
                let head_value = self.head.take().map(|head| {
                    self.tail = None;
                    self.length -= 1;
                    head.data
                });
                return head_value;
            }
        }

        let mut current = self.head.as_mut().map(|node| &mut **node);
        while let Some(node) = current {
            if let Some(existing_node) = &node.next {
                if existing_node.next.is_none() {
                    let tail_value = node.next.take().map(|tail| {
                        self.tail = Some(node as *mut Node<T>);
                        self.length -= 1;
                        tail.data
                    });
                    return tail_value;
                }
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
        match self.head.take() {
            Some(mut node) => {
                let data = node.data;
                self.head = node.next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(data)
            }
            None => None,
        }
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

    /// Inserts a new element with the specified value at the given index in the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The position at which to insert the new element. Must be a non-negative integer.
    /// * `value` - The value to insert into the linked list.
    ///
    /// # Behavior
    ///
    /// * If the index is negative or greater than the length of the list, the function returns without inserting.
    /// * If the index is `0`, the new element is prepended to the list.
    /// * If the index is equal to the length of the list, the new element is appended to the list.
    /// * Otherwise, the new element is inserted at the specified position, and subsequent elements are shifted.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(10);
    /// list.append(20);
    /// list.append(30);
    ///
    /// list.insert(2, 25); // Insert 25 at index 2
    ///
    /// assert_eq!(list.get(0), Some(&10));
    /// assert_eq!(list.get(1), Some(&20));
    /// assert_eq!(list.get(2), Some(&25));
    /// assert_eq!(list.get(3), Some(&30));
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn insert(&mut self, index: i64, value: T) {
        if index.is_negative() || index as u64 >= self.length {
            return;
        }

        if index == 0 {
            self.prepend(value);
            return;
        }

        if index as u64 == self.length {
            self.append(value);
            return;
        }

        let mut current = &mut self.head;
        let mut counter = 0;

        while counter < index - 1 {
            if let Some(ref mut node) = current {
                current = &mut node.next;
            } else {
                return;
            }
            counter += 1;
        }

        if let Some(ref mut node) = current {
            let mut new_node = Box::new(Node::new(value));
            new_node.next = node.next.take();
            node.next = Some(new_node);
        }
    }

    /// Removes the element at the specified index from the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to be removed. Must be a non-negative integer and less than the length of the list.
    ///
    /// # Behavior
    ///
    /// * If the index is out of bounds (negative or greater than or equal to the length of the list), the function returns without making any changes.
    /// * If the index is `0`, the head element is removed.
    /// * If the index is the last element, the tail pointer is updated appropriately.
    /// * For all other indices, the element at the specified index is removed and the list is re-linked.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(10);
    /// list.append(20);
    /// list.append(30);
    /// list.append(40);
    ///
    /// list.remove(2); // Removes the element at index 2 (value 30)
    ///
    /// assert_eq!(list.get(0), Some(&10));
    /// assert_eq!(list.get(1), Some(&20));
    /// assert_eq!(list.get(2), Some(&40));
    /// assert_eq!(list.length, 3);
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn remove(&mut self, index: i64) {
        if index.is_negative() || index as u64 >= self.length {
            return;
        }

        if index == 0 {
            self.pop_first();
            self.length -= 1;
            return;
        }

        let mut current = &mut self.head;
        let mut counter = 0;

        while counter < index - 1 {
            if let Some(ref mut node) = current {
                current = &mut node.next;
            } else {
                return;
            }
            counter += 1;
        }

        if let Some(ref mut node) = current {
            if index as u64 == self.length - 1 {
                if let Some(ref mut last_node) = node.next {
                    Some(last_node).take();
                    self.tail = Some(&mut **node);
                    self.length -= 1;
                    return;
                }
            } else if let Some(ref mut mid_node) = node.next.take() {
                node.next = mid_node.next.take();
            }
            self.length -= 1;
        }
    }

    /// Reverses the linked list in place.
    ///
    /// # Description
    /// This method reverses the order of the nodes in the linked list. After the
    /// operation, the head of the list will be the original tail, and the tail
    /// will be the original head.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = LinkedList::new();
    /// list.append(1);
    /// list.append(2);
    /// list.append(3);
    ///
    /// list.reverse();
    ///
    /// assert_eq!(list.get(0), Some(&3));
    /// assert_eq!(list.get(1), Some(&2));
    /// assert_eq!(list.get(2), Some(&1));
    /// ```
    ///
    /// # Panics
    /// This function does not panic.
    pub fn reverse(&mut self) {
        let mut previous_node = None;
        let mut current_node = self.head.take();

        while let Some(mut node_that_is_iterated) = current_node {
            let next_node = node_that_is_iterated.next.take();
            node_that_is_iterated.next = previous_node;
            previous_node = Some(node_that_is_iterated);
            current_node = next_node;
        }

        self.head = previous_node;
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

    #[test]
    fn test_insert() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        list.insert(0, 10);
        assert_eq!(list.get(0), Some(&10));

        list.append(20);
        list.append(30);

        list.insert(0, 5);
        assert_eq!(list.get(0), Some(&5));
        assert_eq!(list.get(1), Some(&10));
        assert_eq!(list.get(2), Some(&20));
        assert_eq!(list.get(3), Some(&30));

        list.insert(4, 35);
        assert_eq!(list.get(4), Some(&35));

        list.insert(2, 15);
        assert_eq!(list.get(0), Some(&5));
        assert_eq!(list.get(1), Some(&10));
        assert_eq!(list.get(2), Some(&15));
        assert_eq!(list.get(3), Some(&20));
        assert_eq!(list.get(4), Some(&30));
        assert_eq!(list.get(5), Some(&35));

        list.insert(10, 40);
        assert_eq!(list.get(6), None);

        list.insert(-1, 50);
        assert_eq!(list.get(6), None);
    }

    #[test]
    fn test_remove() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();

        list.remove(0);
        assert_eq!(list.length, 0);

        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);

        list.remove(0);
        assert_eq!(list.get(0), Some(&20));
        assert_eq!(list.length, 3);

        list.remove(2);
        assert_eq!(list.get(1), Some(&30));
        assert_eq!(list.get(2), None);
        assert_eq!(list.length, 2);

        list.append(50);
        list.remove(1);
        assert_eq!(list.get(0), Some(&20));
        assert_eq!(list.get(1), Some(&50));
        assert_eq!(list.length, 2);

        list.remove(10);
        assert_eq!(list.length, 2);

        list.remove(-1);
        assert_eq!(list.length, 2);
    }

    #[test]
    fn test_reverse_empty_list() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();
        list.reverse();
        assert_eq!(list.get(0), None);
    }

    #[test]
    fn test_reverse_single_element_list() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();
        list.append(1);
        list.reverse();
        assert_eq!(list.get(0), Some(&1));
    }

    #[test]
    fn test_reverse_multiple_elements_list() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        list.reverse();

        assert_eq!(list.get(0), Some(&3));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&1));
    }

    #[test]
    fn test_reverse_twice() {
        let mut list: KolzoLinkedList<i32> = KolzoLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        list.reverse();
        list.reverse();

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
    }
}
