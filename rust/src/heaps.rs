use crate::Error;

/// A generic binary heap.
pub struct Heap<T> {
    data: Vec<T>,
    less: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T: Clone> Heap<T> {
    /// Creates a new Heap with the given comparison function.
    pub fn new(less: impl Fn(&T, &T) -> bool + 'static) -> Self {
        Self {
            data: Vec::new(),
            less: Box::new(less),
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Pushes a value.
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    /// Pops and returns the top value.
    pub fn pop(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::Empty);
        }
        if self.data.len() == 1 {
            return Ok(self.data.pop().unwrap());
        }
        let root = self.data[0].clone();
        self.data[0] = self.data.pop().unwrap();
        self.sift_down(0);
        Ok(root)
    }

    /// Returns the top value without removing it.
    pub fn peek(&self) -> Result<&T, Error> {
        self.data.first().ok_or(Error::Empty)
    }

    /// Returns a Vec of the heap's data.
    pub fn to_vec(&self) -> Vec<T> {
        self.data.clone()
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if (self.less)(&self.data[index], &self.data[parent]) {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let mut target = index;
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            if left < len && (self.less)(&self.data[left], &self.data[target]) {
                target = left;
            }
            if right < len && (self.less)(&self.data[right], &self.data[target]) {
                target = right;
            }
            if target == index {
                break;
            }
            self.data.swap(index, target);
            index = target;
        }
    }
}

/// A min-heap for ordered types.
pub struct MinHeap<T: Ord + Clone> {
    inner: Heap<T>,
}

impl<T: Ord + Clone> MinHeap<T> {
    /// Creates a new MinHeap.
    pub fn new() -> Self {
        Self {
            inner: Heap::new(|a, b| a < b),
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Pushes a value.
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Pops and returns the minimum value.
    pub fn pop(&mut self) -> Result<T, Error> {
        self.inner.pop()
    }

    /// Returns the minimum value.
    pub fn peek(&self) -> Result<&T, Error> {
        self.inner.peek()
    }

    /// Returns a Vec of the heap's data.
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.to_vec()
    }
}

impl<T: Ord + Clone> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A max-heap for ordered types.
pub struct MaxHeap<T: Ord + Clone> {
    inner: Heap<T>,
}

impl<T: Ord + Clone> MaxHeap<T> {
    /// Creates a new MaxHeap.
    pub fn new() -> Self {
        Self {
            inner: Heap::new(|a, b| a > b),
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Pushes a value.
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Pops and returns the maximum value.
    pub fn pop(&mut self) -> Result<T, Error> {
        self.inner.pop()
    }

    /// Returns the maximum value.
    pub fn peek(&self) -> Result<&T, Error> {
        self.inner.peek()
    }

    /// Returns a Vec of the heap's data.
    pub fn to_vec(&self) -> Vec<T> {
        self.inner.to_vec()
    }
}

impl<T: Ord + Clone> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
