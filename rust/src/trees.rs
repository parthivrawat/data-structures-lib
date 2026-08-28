use crate::Error;

#[derive(Debug)]
struct BstNode<T: Ord + Clone> {
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

/// A binary search tree.
#[derive(Debug)]
pub struct BinarySearchTree<T: Ord + Clone> {
    root: Option<Box<BstNode<T>>>,
    len: usize,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    /// Creates a new BinarySearchTree.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts a value, ignoring duplicates.
    pub fn insert(&mut self, value: T) {
        if self.search(&value) {
            return;
        }
        self.root = Some(Self::insert_node(self.root.take(), value));
        self.len += 1;
    }

    fn insert_node(node: Option<Box<BstNode<T>>>, value: T) -> Box<BstNode<T>> {
        match node {
            None => Box::new(BstNode {
                value,
                left: None,
                right: None,
            }),
            Some(mut n) => {
                if value < n.value {
                    n.left = Some(Self::insert_node(n.left.take(), value));
                } else if value > n.value {
                    n.right = Some(Self::insert_node(n.right.take(), value));
                }
                n
            }
        }
    }

    /// Deletes a value.
    pub fn delete(&mut self, value: &T) -> Result<(), Error> {
        if !self.search(value) {
            return Err(Error::NotFound);
        }
        self.root = Self::delete_node(self.root.take(), value);
        self.len -= 1;
        Ok(())
    }

    fn delete_node(node: Option<Box<BstNode<T>>>, value: &T) -> Option<Box<BstNode<T>>> {
        let mut n = node?;
        if value < &n.value {
            n.left = Self::delete_node(n.left.take(), value);
        } else if value > &n.value {
            n.right = Self::delete_node(n.right.take(), value);
        } else {
            if n.left.is_none() {
                return n.right;
            }
            if n.right.is_none() {
                return n.left;
            }
            let successor = Self::min_node(n.right.as_ref().unwrap()).value.clone();
            n.value = successor;
            n.right = Self::delete_node(n.right.take(), &n.value);
        }
        Some(n)
    }

    fn min_node(node: &BstNode<T>) -> &BstNode<T> {
        match &node.left {
            Some(left) => Self::min_node(left),
            None => node,
        }
    }

    /// Returns true if the value exists.
    pub fn search(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(n) = current {
            if value == &n.value {
                return true;
            } else if value < &n.value {
                current = &n.left;
            } else {
                current = &n.right;
            }
        }
        false
    }

    /// Returns the values in in-order sorted order.
    pub fn in_order(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        Self::in_order_node(&self.root, &mut result);
        result
    }

    fn in_order_node(node: &Option<Box<BstNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            Self::in_order_node(&n.left, result);
            result.push(n.value.clone());
            Self::in_order_node(&n.right, result);
        }
    }
}

impl<T: Ord + Clone> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct AvlNode<T: Ord + Clone> {
    value: T,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
    height: i32,
}

/// A self-balancing AVL tree.
#[derive(Debug)]
pub struct AvlTree<T: Ord + Clone> {
    root: Option<Box<AvlNode<T>>>,
    len: usize,
}

impl<T: Ord + Clone> AvlTree<T> {
    /// Creates a new AvlTree.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the height of the tree.
    pub fn height(&self) -> i32 {
        Self::height_node(&self.root)
    }

    fn height_node(node: &Option<Box<AvlNode<T>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => n.height,
        }
    }

    fn update_height(n: &mut AvlNode<T>) {
        n.height = 1
            + std::cmp::max(
                Self::height_node(&n.left),
                Self::height_node(&n.right),
            );
    }

    fn balance_factor(n: &AvlNode<T>) -> i32 {
        Self::height_node(&n.left) - Self::height_node(&n.right)
    }

    fn right_rotate(mut y: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();
        x.right = Some(y);
        x.right.as_mut().unwrap().left = t2;
        Self::update_height(x.right.as_mut().unwrap());
        Self::update_height(&mut x);
        x
    }

    fn left_rotate(mut x: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();
        y.left = Some(x);
        y.left.as_mut().unwrap().right = t2;
        Self::update_height(y.left.as_mut().unwrap());
        Self::update_height(&mut y);
        y
    }

    /// Inserts a value, ignoring duplicates.
    pub fn insert(&mut self, value: T) {
        if self.search(&value) {
            return;
        }
        self.root = Some(Self::insert_node(self.root.take(), value));
        self.len += 1;
    }

    fn insert_node(node: Option<Box<AvlNode<T>>>, value: T) -> Box<AvlNode<T>> {
        match node {
            None => Box::new(AvlNode {
                value,
                left: None,
                right: None,
                height: 1,
            }),
            Some(mut n) => {
                if value < n.value {
                    n.left = Some(Self::insert_node(n.left.take(), value.clone()));
                } else if value > n.value {
                    n.right = Some(Self::insert_node(n.right.take(), value.clone()));
                } else {
                    return n;
                }
                Self::update_height(&mut n);
                let balance = Self::balance_factor(&n);

                if balance > 1 && value < n.left.as_ref().unwrap().value {
                    return Self::right_rotate(n);
                }
                if balance < -1 && value > n.right.as_ref().unwrap().value {
                    return Self::left_rotate(n);
                }
                if balance > 1 && value > n.left.as_ref().unwrap().value {
                    n.left = Some(Self::left_rotate(n.left.take().unwrap()));
                    return Self::right_rotate(n);
                }
                if balance < -1 && value < n.right.as_ref().unwrap().value {
                    n.right = Some(Self::right_rotate(n.right.take().unwrap()));
                    return Self::left_rotate(n);
                }
                n
            }
        }
    }

    /// Deletes a value.
    pub fn delete(&mut self, value: &T) -> Result<(), Error> {
        if !self.search(value) {
            return Err(Error::NotFound);
        }
        self.root = Self::delete_node(self.root.take(), value);
        self.len -= 1;
        Ok(())
    }

    fn delete_node(node: Option<Box<AvlNode<T>>>, value: &T) -> Option<Box<AvlNode<T>>> {
        let mut n = node?;
        if value < &n.value {
            n.left = Self::delete_node(n.left.take(), value);
        } else if value > &n.value {
            n.right = Self::delete_node(n.right.take(), value);
        } else {
            if n.left.is_none() || n.right.is_none() {
                let temp = if n.left.is_some() { n.left } else { n.right };
                return temp;
            }
            let successor = Self::min_value_node(n.right.as_ref().unwrap()).value.clone();
            n.value = successor;
            n.right = Self::delete_node(n.right.take(), &n.value);
        }

        Self::update_height(&mut n);
        let balance = Self::balance_factor(&n);

        if balance > 1 && Self::balance_factor(n.left.as_ref().unwrap()) >= 0 {
            return Some(Self::right_rotate(n));
        }
        if balance > 1 && Self::balance_factor(n.left.as_ref().unwrap()) < 0 {
            n.left = Some(Self::left_rotate(n.left.take().unwrap()));
            return Some(Self::right_rotate(n));
        }
        if balance < -1 && Self::balance_factor(n.right.as_ref().unwrap()) <= 0 {
            return Some(Self::left_rotate(n));
        }
        if balance < -1 && Self::balance_factor(n.right.as_ref().unwrap()) > 0 {
            n.right = Some(Self::right_rotate(n.right.take().unwrap()));
            return Some(Self::left_rotate(n));
        }
        Some(n)
    }

    fn min_value_node(node: &AvlNode<T>) -> &AvlNode<T> {
        match &node.left {
            Some(left) => Self::min_value_node(left),
            None => node,
        }
    }

    /// Returns true if the value exists.
    pub fn search(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(n) = current {
            if value == &n.value {
                return true;
            } else if value < &n.value {
                current = &n.left;
            } else {
                current = &n.right;
            }
        }
        false
    }

    /// Returns the values in in-order sorted order.
    pub fn in_order(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        Self::in_order_node(&self.root, &mut result);
        result
    }

    fn in_order_node(node: &Option<Box<AvlNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            Self::in_order_node(&n.left, result);
            result.push(n.value.clone());
            Self::in_order_node(&n.right, result);
        }
    }
}

impl<T: Ord + Clone> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
