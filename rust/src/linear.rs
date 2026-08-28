use crate::Error;

/// A dynamic array backed by a Vec.
#[derive(Debug, Clone)]
pub struct DynamicArray<T> {
    data: Vec<T>,
}

impl<T> DynamicArray<T> {
    /// Creates a new DynamicArray with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new DynamicArray with a default capacity.
    pub fn new() -> Self {
        Self::with_capacity(10)
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Appends a value.
    pub fn append(&mut self, value: T) {
        self.data.push(value);
    }

    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Error> {
        if index > self.data.len() {
            return Err(Error::OutOfBounds);
        }
        self.data.insert(index, value);
        Ok(())
    }

    /// Removes the first occurrence of a value.
    pub fn remove(&mut self, value: &T) -> Result<(), Error>
    where
        T: PartialEq,
    {
        if let Some(index) = self.data.iter().position(|x| x == value) {
            self.data.remove(index);
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    /// Removes and returns the item at the given index, defaulting to the last.
    pub fn pop(&mut self, index: Option<usize>) -> Result<T, Error> {
        let idx = index.unwrap_or(self.data.len().saturating_sub(1));
        if idx >= self.data.len() {
            return Err(Error::OutOfBounds);
        }
        Ok(self.data.remove(idx))
    }

    /// Returns the item at index.
    pub fn get(&self, index: usize) -> Result<&T, Error> {
        self.data.get(index).ok_or(Error::OutOfBounds)
    }

    /// Sets the item at index.
    pub fn set(&mut self, index: usize, value: T) -> Result<(), Error> {
        if index >= self.data.len() {
            return Err(Error::OutOfBounds);
        }
        self.data[index] = value;
        Ok(())
    }

    /// Returns a Vec of all elements.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data.clone()
    }
}

impl<T> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct SNode<T> {
    value: T,
    next: Option<usize>,
}

/// A singly linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    nodes: Vec<SNode<T>>,
    head: Option<usize>,
    free: Vec<usize>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    /// Creates a new SinglyLinkedList.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            free: Vec::new(),
            len: 0,
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn alloc(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop() {
            self.nodes[index] = SNode { value, next: None };
            index
        } else {
            self.nodes.push(SNode { value, next: None });
            self.nodes.len() - 1
        }
    }

    fn free_node(&mut self, index: usize) {
        self.free.push(index);
    }

    fn node_at(&self, index: usize) -> Result<usize, Error> {
        if index >= self.len {
            return Err(Error::OutOfBounds);
        }
        let mut current = self.head.unwrap();
        for _ in 0..index {
            current = self.nodes[current].next.unwrap();
        }
        Ok(current)
    }

    /// Appends a value.
    pub fn append(&mut self, value: T) {
        let index = self.alloc(value);
        if let Some(tail_index) = self.tail() {
            self.nodes[tail_index].next = Some(index);
        } else {
            self.head = Some(index);
        }
        self.len += 1;
    }

    fn tail(&self) -> Option<usize> {
        let mut current = self.head?;
        while let Some(next) = self.nodes[current].next {
            current = next;
        }
        Some(current)
    }

    /// Prepends a value.
    pub fn prepend(&mut self, value: T) {
        let index = self.alloc(value);
        self.nodes[index].next = self.head;
        self.head = Some(index);
        self.len += 1;
    }

    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Error> {
        if index > self.len {
            return Err(Error::OutOfBounds);
        }
        if index == 0 {
            self.prepend(value);
            return Ok(());
        }
        let prev = self.node_at(index - 1)?;
        let new_index = self.alloc(value);
        self.nodes[new_index].next = self.nodes[prev].next;
        self.nodes[prev].next = Some(new_index);
        self.len += 1;
        Ok(())
    }

    /// Removes the first occurrence of a value.
    pub fn remove(&mut self, value: &T) -> Result<(), Error>
    where
        T: PartialEq,
    {
        let head = self.head.ok_or(Error::NotFound)?;
        if &self.nodes[head].value == value {
            let next = self.nodes[head].next;
            self.free_node(head);
            self.head = next;
            self.len -= 1;
            return Ok(());
        }
        let mut current = head;
        while let Some(next) = self.nodes[current].next {
            if &self.nodes[next].value == value {
                self.nodes[current].next = self.nodes[next].next;
                self.free_node(next);
                self.len -= 1;
                return Ok(());
            }
            current = next;
        }
        Err(Error::NotFound)
    }

    /// Removes and returns the item at the given index, defaulting to the last.
    pub fn pop(&mut self, index: Option<usize>) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::Empty);
        }
        let idx = index.unwrap_or(self.len - 1);
        if idx >= self.len {
            return Err(Error::OutOfBounds);
        }
        if idx == 0 {
            let head = self.head.unwrap();
            let value = std::mem::replace(&mut self.nodes[head].value, unsafe { std::mem::zeroed() });
            let next = self.nodes[head].next;
            self.free_node(head);
            self.head = next;
            self.len -= 1;
            return Ok(value);
        }
        let prev = self.node_at(idx - 1)?;
        let current = self.nodes[prev].next.unwrap();
        let value = std::mem::replace(&mut self.nodes[current].value, unsafe { std::mem::zeroed() });
        self.nodes[prev].next = self.nodes[current].next;
        self.free_node(current);
        self.len -= 1;
        Ok(value)
    }

    /// Returns the item at index.
    pub fn get(&self, index: usize) -> Result<&T, Error> {
        let node = self.node_at(index)?;
        Ok(&self.nodes[node].value)
    }

    /// Returns a mutable reference to the item at index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, Error> {
        let node = self.node_at(index)?;
        Ok(&mut self.nodes[node].value)
    }

    /// Returns the index of the first occurrence of a value.
    pub fn find(&self, value: &T) -> Result<usize, Error>
    where
        T: PartialEq,
    {
        let mut current = self.head;
        let mut i = 0;
        while let Some(idx) = current {
            if &self.nodes[idx].value == value {
                return Ok(i);
            }
            current = self.nodes[idx].next;
            i += 1;
        }
        Err(Error::NotFound)
    }

    /// Returns a Vec of all values.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::with_capacity(self.len);
        let mut current = self.head;
        while let Some(idx) = current {
            result.push(self.nodes[idx].value.clone());
            current = self.nodes[idx].next;
        }
        result
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct DNode<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

/// A doubly linked list.
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    nodes: Vec<DNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    free: Vec<usize>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new DoublyLinkedList.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
            free: Vec::new(),
            len: 0,
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn alloc(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop() {
            self.nodes[index] = DNode {
                value,
                prev: None,
                next: None,
            };
            index
        } else {
            self.nodes.push(DNode {
                value,
                prev: None,
                next: None,
            });
            self.nodes.len() - 1
        }
    }

    fn free_node(&mut self, index: usize) {
        self.free.push(index);
    }

    fn node_at(&self, index: usize) -> Result<usize, Error> {
        if index >= self.len {
            return Err(Error::OutOfBounds);
        }
        if index <= self.len / 2 {
            let mut current = self.head.unwrap();
            for _ in 0..index {
                current = self.nodes[current].next.unwrap();
            }
            Ok(current)
        } else {
            let mut current = self.tail.unwrap();
            for _ in 0..self.len - index - 1 {
                current = self.nodes[current].prev.unwrap();
            }
            Ok(current)
        }
    }

    /// Appends a value.
    pub fn append(&mut self, value: T) {
        let index = self.alloc(value);
        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(index);
            self.nodes[index].prev = Some(tail);
            self.tail = Some(index);
        } else {
            self.head = Some(index);
            self.tail = Some(index);
        }
        self.len += 1;
    }

    /// Prepends a value.
    pub fn prepend(&mut self, value: T) {
        let index = self.alloc(value);
        if let Some(head) = self.head {
            self.nodes[head].prev = Some(index);
            self.nodes[index].next = Some(head);
            self.head = Some(index);
        } else {
            self.head = Some(index);
            self.tail = Some(index);
        }
        self.len += 1;
    }

    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Error> {
        if index > self.len {
            return Err(Error::OutOfBounds);
        }
        if index == 0 {
            self.prepend(value);
            return Ok(());
        }
        if index == self.len {
            self.append(value);
            return Ok(());
        }
        let next = self.node_at(index)?;
        let prev = self.nodes[next].prev.unwrap();
        let new_index = self.alloc(value);
        self.nodes[new_index].prev = Some(prev);
        self.nodes[new_index].next = Some(next);
        self.nodes[prev].next = Some(new_index);
        self.nodes[next].prev = Some(new_index);
        self.len += 1;
        Ok(())
    }

    /// Removes the first occurrence of a value.
    pub fn remove(&mut self, value: &T) -> Result<(), Error>
    where
        T: PartialEq,
    {
        let mut current = self.head;
        while let Some(idx) = current {
            if &self.nodes[idx].value == value {
                self.remove_node(idx);
                return Ok(());
            }
            current = self.nodes[idx].next;
        }
        Err(Error::NotFound)
    }

    fn remove_node(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            self.tail = prev;
        }
        self.free_node(index);
        self.len -= 1;
    }

    /// Removes and returns the item at the given index, defaulting to the last.
    pub fn pop(&mut self, index: Option<usize>) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::Empty);
        }
        let idx = index.unwrap_or(self.len - 1);
        if idx >= self.len {
            return Err(Error::OutOfBounds);
        }
        let node = self.node_at(idx)?;
        let value = std::mem::replace(&mut self.nodes[node].value, unsafe { std::mem::zeroed() });
        self.remove_node(node);
        Ok(value)
    }

    /// Returns the item at index.
    pub fn get(&self, index: usize) -> Result<&T, Error> {
        let node = self.node_at(index)?;
        Ok(&self.nodes[node].value)
    }

    /// Returns a mutable reference to the item at index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, Error> {
        let node = self.node_at(index)?;
        Ok(&mut self.nodes[node].value)
    }

    /// Returns a Vec of all values.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::with_capacity(self.len);
        let mut current = self.head;
        while let Some(idx) = current {
            result.push(self.nodes[idx].value.clone());
            current = self.nodes[idx].next;
        }
        result
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct CNode<T> {
    value: T,
    next: Option<usize>,
}

/// A circular linked list.
#[derive(Debug)]
pub struct CircularLinkedList<T> {
    nodes: Vec<CNode<T>>,
    tail: Option<usize>,
    free: Vec<usize>,
    len: usize,
}

impl<T> CircularLinkedList<T> {
    /// Creates a new CircularLinkedList.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            tail: None,
            free: Vec::new(),
            len: 0,
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn alloc(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop() {
            self.nodes[index] = CNode { value, next: None };
            index
        } else {
            self.nodes.push(CNode { value, next: None });
            self.nodes.len() - 1
        }
    }

    fn free_node(&mut self, index: usize) {
        self.free.push(index);
    }

    /// Appends a value.
    pub fn append(&mut self, value: T) {
        let index = self.alloc(value);
        if let Some(tail) = self.tail {
            let head = self.nodes[tail].next.unwrap();
            self.nodes[tail].next = Some(index);
            self.nodes[index].next = Some(head);
            self.tail = Some(index);
        } else {
            self.nodes[index].next = Some(index);
            self.tail = Some(index);
        }
        self.len += 1;
    }

    /// Prepends a value.
    pub fn prepend(&mut self, value: T) {
        let index = self.alloc(value);
        if let Some(tail) = self.tail {
            let head = self.nodes[tail].next.unwrap();
            self.nodes[tail].next = Some(index);
            self.nodes[index].next = Some(head);
        } else {
            self.nodes[index].next = Some(index);
            self.tail = Some(index);
        }
        self.len += 1;
    }

    /// Removes the first occurrence of a value.
    pub fn remove(&mut self, value: &T) -> Result<(), Error>
    where
        T: PartialEq,
    {
        let tail = self.tail.ok_or(Error::NotFound)?;
        let head = self.nodes[tail].next.unwrap();
        if &self.nodes[head].value == value {
            if head == tail {
                self.tail = None;
            } else {
                self.nodes[tail].next = self.nodes[head].next;
            }
            self.free_node(head);
            self.len -= 1;
            return Ok(());
        }
        let mut current = head;
        for _ in 0..self.len - 1 {
            if &self.nodes[self.nodes[current].next.unwrap()].value == value {
                let to_remove = self.nodes[current].next.unwrap();
                let after = self.nodes[to_remove].next;
                self.nodes[current].next = after;
                if to_remove == tail {
                    self.tail = Some(current);
                }
                self.free_node(to_remove);
                self.len -= 1;
                return Ok(());
            }
            current = self.nodes[current].next.unwrap();
        }
        Err(Error::NotFound)
    }

    /// Returns a Vec of all values.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        if self.is_empty() {
            return Vec::new();
        }
        let mut result = Vec::with_capacity(self.len);
        let tail = self.tail.unwrap();
        let start = self.nodes[tail].next.unwrap();
        let mut current = start;
        loop {
            result.push(self.nodes[current].value.clone());
            current = self.nodes[current].next.unwrap();
            if current == start {
                break;
            }
        }
        result
    }
}

impl<T> Default for CircularLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A LIFO stack.
#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new Stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Pushes a value.
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Pops and returns the top value.
    pub fn pop(&mut self) -> Result<T, Error> {
        self.items.pop().ok_or(Error::Empty)
    }

    /// Returns the top value without removing it.
    pub fn peek(&self) -> Result<&T, Error> {
        self.items.last().ok_or(Error::Empty)
    }

    /// Returns a Vec from top to bottom.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.items.iter().rev().cloned().collect()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A FIFO queue.
#[derive(Debug, Clone)]
pub struct Queue<T> {
    items: std::collections::VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates a new Queue.
    pub fn new() -> Self {
        Self {
            items: std::collections::VecDeque::new(),
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Enqueues a value.
    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    /// Dequeues and returns the front value.
    pub fn dequeue(&mut self) -> Result<T, Error> {
        self.items.pop_front().ok_or(Error::Empty)
    }

    /// Returns the front value without removing it.
    pub fn peek(&self) -> Result<&T, Error> {
        self.items.front().ok_or(Error::Empty)
    }

    /// Returns a Vec of all values.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.items.iter().cloned().collect()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
