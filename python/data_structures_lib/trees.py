'''Tree-based data structures.'''

from typing import Generic, Iterable, Iterator, Optional, TypeVar

T = TypeVar('T')

__all__ = ['BinarySearchTree', 'AVLTree']


class _BSTNode(Generic[T]):
    __slots__ = ('value', 'left', 'right')

    def __init__(self, value: T) -> None:
        self.value = value
        self.left: Optional['_BSTNode[T]'] = None
        self.right: Optional['_BSTNode[T]'] = None


class BinarySearchTree(Generic[T]):
    '''A binary search tree.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._root: Optional[_BSTNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.insert(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def insert(self, value: T) -> None:
        '''Insert a value, ignoring duplicates.'''
        if value not in self:
            self._root = self._insert(self._root, value)
            self._size += 1

    def _insert(self, node: Optional[_BSTNode[T]], value: T) -> _BSTNode[T]:
        if node is None:
            return _BSTNode(value)
        if value < node.value:
            node.left = self._insert(node.left, value)
        elif value > node.value:
            node.right = self._insert(node.right, value)
        return node

    def delete(self, value: T) -> None:
        '''Delete a value.'''
        if value not in self:
            raise ValueError(f'{value!r} not in tree')
        self._root = self._delete(self._root, value)
        self._size -= 1

    def _delete(self, node: Optional[_BSTNode[T]], value: T) -> Optional[_BSTNode[T]]:
        if node is None:
            return None
        if value < node.value:
            node.left = self._delete(node.left, value)
        elif value > node.value:
            node.right = self._delete(node.right, value)
        else:
            if node.left is None:
                return node.right
            if node.right is None:
                return node.left
            successor = self._min_node(node.right)
            node.value = successor.value
            node.right = self._delete(node.right, successor.value)
        return node

    def _min_node(self, node: _BSTNode[T]) -> _BSTNode[T]:
        while node.left is not None:
            node = node.left
        return node

    def search(self, value: T) -> bool:
        '''Return True if value is in the tree.'''
        node = self._root
        while node is not None:
            if value == node.value:
                return True
            if value < node.value:
                node = node.left
            else:
                node = node.right
        return False

    def __contains__(self, value: T) -> bool:
        return self.search(value)

    def __iter__(self) -> Iterator[T]:
        return self._inorder(self._root)

    def _inorder(self, node: Optional[_BSTNode[T]]) -> Iterator[T]:
        if node is not None:
            yield from self._inorder(node.left)
            yield node.value
            yield from self._inorder(node.right)

    def __repr__(self) -> str:
        return f'BinarySearchTree({list(self)})'


class _AVLNode(Generic[T]):
    __slots__ = ('value', 'left', 'right', 'height')

    def __init__(self, value: T) -> None:
        self.value = value
        self.left: Optional['_AVLNode[T]'] = None
        self.right: Optional['_AVLNode[T]'] = None
        self.height = 1


class AVLTree(Generic[T]):
    '''A self-balancing AVL tree.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._root: Optional[_AVLNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.insert(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def height(self) -> int:
        '''Return the height of the tree.'''
        return self._height(self._root)

    def _height(self, node: Optional[_AVLNode[T]]) -> int:
        if node is None:
            return 0
        return node.height

    def _update_height(self, node: _AVLNode[T]) -> None:
        node.height = 1 + max(self._height(node.left), self._height(node.right))

    def _balance_factor(self, node: _AVLNode[T]) -> int:
        return self._height(node.left) - self._height(node.right)

    def _right_rotate(self, y: _AVLNode[T]) -> _AVLNode[T]:
        x = y.left
        t2 = x.right
        x.right = y
        y.left = t2
        self._update_height(y)
        self._update_height(x)
        return x

    def _left_rotate(self, x: _AVLNode[T]) -> _AVLNode[T]:
        y = x.right
        t2 = y.left
        y.left = x
        x.right = t2
        self._update_height(x)
        self._update_height(y)
        return y

    def insert(self, value: T) -> None:
        '''Insert a value, ignoring duplicates.'''
        if value not in self:
            self._root = self._insert(self._root, value)
            self._size += 1

    def _insert(self, node: Optional[_AVLNode[T]], value: T) -> _AVLNode[T]:
        if node is None:
            return _AVLNode(value)
        if value < node.value:
            node.left = self._insert(node.left, value)
        elif value > node.value:
            node.right = self._insert(node.right, value)
        else:
            return node

        self._update_height(node)
        balance = self._balance_factor(node)

        if balance > 1 and value < node.left.value:
            return self._right_rotate(node)
        if balance < -1 and value > node.right.value:
            return self._left_rotate(node)
        if balance > 1 and value > node.left.value:
            node.left = self._left_rotate(node.left)
            return self._right_rotate(node)
        if balance < -1 and value < node.right.value:
            node.right = self._right_rotate(node.right)
            return self._left_rotate(node)
        return node

    def delete(self, value: T) -> None:
        '''Delete a value.'''
        if value not in self:
            raise ValueError(f'{value!r} not in tree')
        self._root = self._delete(self._root, value)
        self._size -= 1

    def _delete(self, node: Optional[_AVLNode[T]], value: T) -> Optional[_AVLNode[T]]:
        if node is None:
            return None
        if value < node.value:
            node.left = self._delete(node.left, value)
        elif value > node.value:
            node.right = self._delete(node.right, value)
        else:
            if node.left is None or node.right is None:
                temp = node.left if node.left is not None else node.right
                if temp is None:
                    return None
                return temp
            successor = self._min_value_node(node.right)
            node.value = successor.value
            node.right = self._delete(node.right, successor.value)

        if node is None:
            return None

        self._update_height(node)
        balance = self._balance_factor(node)

        if balance > 1 and self._balance_factor(node.left) >= 0:
            return self._right_rotate(node)
        if balance > 1 and self._balance_factor(node.left) < 0:
            node.left = self._left_rotate(node.left)
            return self._right_rotate(node)
        if balance < -1 and self._balance_factor(node.right) <= 0:
            return self._left_rotate(node)
        if balance < -1 and self._balance_factor(node.right) > 0:
            node.right = self._right_rotate(node.right)
            return self._left_rotate(node)
        return node

    def _min_value_node(self, node: _AVLNode[T]) -> _AVLNode[T]:
        while node.left is not None:
            node = node.left
        return node

    def search(self, value: T) -> bool:
        '''Return True if value is in the tree.'''
        node = self._root
        while node is not None:
            if value == node.value:
                return True
            if value < node.value:
                node = node.left
            else:
                node = node.right
        return False

    def __contains__(self, value: T) -> bool:
        return self.search(value)

    def __iter__(self) -> Iterator[T]:
        return self._inorder(self._root)

    def _inorder(self, node: Optional[_AVLNode[T]]) -> Iterator[T]:
        if node is not None:
            yield from self._inorder(node.left)
            yield node.value
            yield from self._inorder(node.right)

    def __repr__(self) -> str:
        return f'AVLTree({list(self)})'
