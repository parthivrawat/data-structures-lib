package datastructures

// bstNode is a binary search tree node.
type bstNode[T Ordered] struct {
	value T
	left  *bstNode[T]
	right *bstNode[T]
}

// BinarySearchTree is a binary search tree.
type BinarySearchTree[T Ordered] struct {
	root *bstNode[T]
	size int
}

// NewBinarySearchTree creates a new BinarySearchTree.
func NewBinarySearchTree[T Ordered]() *BinarySearchTree[T] {
	return &BinarySearchTree[T]{}
}

// Len returns the number of elements.
func (t *BinarySearchTree[T]) Len() int { return t.size }

// IsEmpty returns true if the tree is empty.
func (t *BinarySearchTree[T]) IsEmpty() bool { return t.size == 0 }

// Insert adds a value, ignoring duplicates.
func (t *BinarySearchTree[T]) Insert(value T) {
	if t.Search(value) {
		return
	}
	t.root = t.insert(t.root, value)
	t.size++
}

func (t *BinarySearchTree[T]) insert(node *bstNode[T], value T) *bstNode[T] {
	if node == nil {
		return &bstNode[T]{value: value}
	}
	if value < node.value {
		node.left = t.insert(node.left, value)
	} else if value > node.value {
		node.right = t.insert(node.right, value)
	}
	return node
}

// Delete removes a value.
func (t *BinarySearchTree[T]) Delete(value T) error {
	if !t.Search(value) {
		return ErrNotFound
	}
	t.root = t.delete(t.root, value)
	t.size--
	return nil
}

func (t *BinarySearchTree[T]) delete(node *bstNode[T], value T) *bstNode[T] {
	if node == nil {
		return nil
	}
	if value < node.value {
		node.left = t.delete(node.left, value)
	} else if value > node.value {
		node.right = t.delete(node.right, value)
	} else {
		if node.left == nil {
			return node.right
		}
		if node.right == nil {
			return node.left
		}
		successor := t.minNode(node.right)
		node.value = successor.value
		node.right = t.delete(node.right, successor.value)
	}
	return node
}

func (t *BinarySearchTree[T]) minNode(node *bstNode[T]) *bstNode[T] {
	for node.left != nil {
		node = node.left
	}
	return node
}

// Search returns true if the value exists.
func (t *BinarySearchTree[T]) Search(value T) bool {
	node := t.root
	for node != nil {
		if value == node.value {
			return true
		}
		if value < node.value {
			node = node.left
		} else {
			node = node.right
		}
	}
	return false
}

// InOrder returns the values in sorted order.
func (t *BinarySearchTree[T]) InOrder() []T {
	result := make([]T, 0, t.size)
	t.inOrder(t.root, &result)
	return result
}

func (t *BinarySearchTree[T]) inOrder(node *bstNode[T], result *[]T) {
	if node == nil {
		return
	}
	t.inOrder(node.left, result)
	*result = append(*result, node.value)
	t.inOrder(node.right, result)
}

// avlNode is an AVL tree node.
type avlNode[T Ordered] struct {
	value  T
	left   *avlNode[T]
	right  *avlNode[T]
	height int
}

// AVLTree is a self-balancing AVL tree.
type AVLTree[T Ordered] struct {
	root *avlNode[T]
	size int
}

// NewAVLTree creates a new AVLTree.
func NewAVLTree[T Ordered]() *AVLTree[T] {
	return &AVLTree[T]{}
}

// Len returns the number of elements.
func (t *AVLTree[T]) Len() int { return t.size }

// IsEmpty returns true if the tree is empty.
func (t *AVLTree[T]) IsEmpty() bool { return t.size == 0 }

// Height returns the height of the tree.
func (t *AVLTree[T]) Height() int {
	return t.height(t.root)
}

func (t *AVLTree[T]) height(node *avlNode[T]) int {
	if node == nil {
		return 0
	}
	return node.height
}

func (t *AVLTree[T]) updateHeight(node *avlNode[T]) {
	node.height = 1 + max(t.height(node.left), t.height(node.right))
}

func (t *AVLTree[T]) balanceFactor(node *avlNode[T]) int {
	return t.height(node.left) - t.height(node.right)
}

func (t *AVLTree[T]) rightRotate(y *avlNode[T]) *avlNode[T] {
	x := y.left
	t2 := x.right
	x.right = y
	y.left = t2
	t.updateHeight(y)
	t.updateHeight(x)
	return x
}

func (t *AVLTree[T]) leftRotate(x *avlNode[T]) *avlNode[T] {
	y := x.right
	t2 := y.left
	y.left = x
	x.right = t2
	t.updateHeight(x)
	t.updateHeight(y)
	return y
}

// Insert adds a value, ignoring duplicates.
func (t *AVLTree[T]) Insert(value T) {
	if t.Search(value) {
		return
	}
	t.root = t.insert(t.root, value)
	t.size++
}

func (t *AVLTree[T]) insert(node *avlNode[T], value T) *avlNode[T] {
	if node == nil {
		return &avlNode[T]{value: value, height: 1}
	}
	if value < node.value {
		node.left = t.insert(node.left, value)
	} else if value > node.value {
		node.right = t.insert(node.right, value)
	} else {
		return node
	}

	t.updateHeight(node)
	balance := t.balanceFactor(node)

	if balance > 1 && value < node.left.value {
		return t.rightRotate(node)
	}
	if balance < -1 && value > node.right.value {
		return t.leftRotate(node)
	}
	if balance > 1 && value > node.left.value {
		node.left = t.leftRotate(node.left)
		return t.rightRotate(node)
	}
	if balance < -1 && value < node.right.value {
		node.right = t.rightRotate(node.right)
		return t.leftRotate(node)
	}
	return node
}

// Delete removes a value.
func (t *AVLTree[T]) Delete(value T) error {
	if !t.Search(value) {
		return ErrNotFound
	}
	t.root = t.delete(t.root, value)
	t.size--
	return nil
}

func (t *AVLTree[T]) delete(node *avlNode[T], value T) *avlNode[T] {
	if node == nil {
		return nil
	}
	if value < node.value {
		node.left = t.delete(node.left, value)
	} else if value > node.value {
		node.right = t.delete(node.right, value)
	} else {
		if node.left == nil || node.right == nil {
			temp := node.left
			if temp == nil {
				temp = node.right
			}
			if temp == nil {
				return nil
			}
			return temp
		}
		successor := t.minValueNode(node.right)
		node.value = successor.value
		node.right = t.delete(node.right, successor.value)
	}

	if node == nil {
		return nil
	}

	t.updateHeight(node)
	balance := t.balanceFactor(node)

	if balance > 1 && t.balanceFactor(node.left) >= 0 {
		return t.rightRotate(node)
	}
	if balance > 1 && t.balanceFactor(node.left) < 0 {
		node.left = t.leftRotate(node.left)
		return t.rightRotate(node)
	}
	if balance < -1 && t.balanceFactor(node.right) <= 0 {
		return t.leftRotate(node)
	}
	if balance < -1 && t.balanceFactor(node.right) > 0 {
		node.right = t.rightRotate(node.right)
		return t.leftRotate(node)
	}
	return node
}

func (t *AVLTree[T]) minValueNode(node *avlNode[T]) *avlNode[T] {
	for node.left != nil {
		node = node.left
	}
	return node
}

// Search returns true if the value exists.
func (t *AVLTree[T]) Search(value T) bool {
	node := t.root
	for node != nil {
		if value == node.value {
			return true
		}
		if value < node.value {
			node = node.left
		} else {
			node = node.right
		}
	}
	return false
}

// InOrder returns the values in sorted order.
func (t *AVLTree[T]) InOrder() []T {
	result := make([]T, 0, t.size)
	t.inOrder(t.root, &result)
	return result
}

func (t *AVLTree[T]) inOrder(node *avlNode[T], result *[]T) {
	if node == nil {
		return
	}
	t.inOrder(node.left, result)
	*result = append(*result, node.value)
	t.inOrder(node.right, result)
}
