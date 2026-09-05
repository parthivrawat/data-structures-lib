'''Linear data structures.'''

from typing import Generic, Iterable, Iterator, List, Optional, TypeVar, cast

from .exceptions import EmptyStructureError

T = TypeVar('T')

__all__ = [
    'DynamicArray',
    'SinglyLinkedList',
    'DoublyLinkedList',
    'CircularLinkedList',
    'Stack',
    'Queue',
]


class DynamicArray(Generic[T]):
    '''A dynamic array with automatic resizing.'''

    def __init__(self, initial_capacity: int = 10) -> None:
        if initial_capacity <= 0:
            raise ValueError('initial_capacity must be positive')
        self._capacity = initial_capacity
        self._size = 0
        self._data: List[T] = cast(List[T], [None] * self._capacity)

    def __len__(self) -> int:
        return self._size

    @property
    def capacity(self) -> int:
        '''Current internal capacity.'''
        return self._capacity

    def is_empty(self) -> bool:
        '''Return True if the array is empty.'''
        return self._size == 0

    def append(self, value: T) -> None:
        '''Append a value to the end.'''
        if self._size == self._capacity:
            self._resize(2 * self._capacity)
        self._data[self._size] = value
        self._size += 1

    def insert(self, index: int, value: T) -> None:
        '''Insert a value at the given index.'''
        if not 0 <= index <= self._size:
            raise IndexError('index out of range')
        if self._size == self._capacity:
            self._resize(2 * self._capacity)
        for i in range(self._size, index, -1):
            self._data[i] = self._data[i - 1]
        self._data[index] = value
        self._size += 1

    def remove(self, value: T) -> None:
        '''Remove the first occurrence of value.'''
        for i in range(self._size):
            if self._data[i] == value:
                self.pop(i)
                return
        raise ValueError(f'{value!r} not in array')

    def pop(self, index: int = -1) -> T:
        '''Remove and return the item at index.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty array')
        if index < 0:
            index += self._size
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        value = self._data[index]
        for i in range(index, self._size - 1):
            self._data[i] = self._data[i + 1]
        self._size -= 1
        if self._size > 0 and self._size == self._capacity // 4 and self._capacity > 10:
            self._resize(self._capacity // 2)
        return value

    def __getitem__(self, index: int) -> T:
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        return self._data[index]

    def __setitem__(self, index: int, value: T) -> None:
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        self._data[index] = value

    def __iter__(self) -> Iterator[T]:
        for i in range(self._size):
            yield self._data[i]

    def __contains__(self, value: T) -> bool:
        for i in range(self._size):
            if self._data[i] == value:
                return True
        return False

    def __repr__(self) -> str:
        return f'DynamicArray({list(self)})'

    def _resize(self, new_capacity: int) -> None:
        new_data: List[T] = cast(List[T], [None] * new_capacity)
        for i in range(self._size):
            new_data[i] = self._data[i]
        self._data = new_data
        self._capacity = new_capacity


class _SNode(Generic[T]):
    __slots__ = ('value', 'next_node')

    def __init__(self, value: T) -> None:
        self.value = value
        self.next_node: Optional['_SNode[T]'] = None


class SinglyLinkedList(Generic[T]):
    '''A singly linked list.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._head: Optional[_SNode[T]] = None
        self._tail: Optional[_SNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.append(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def _node_at(self, index: int) -> _SNode[T]:
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        current = self._head
        for _ in range(index):
            current = current.next_node
        return current

    def append(self, value: T) -> None:
        '''Append a value to the end.'''
        node = _SNode(value)
        if not self._tail:
            self._head = node
        else:
            self._tail.next_node = node
        self._tail = node
        self._size += 1

    def prepend(self, value: T) -> None:
        '''Insert a value at the front.'''
        node = _SNode(value)
        node.next_node = self._head
        self._head = node
        if not self._tail:
            self._tail = node
        self._size += 1

    def insert(self, index: int, value: T) -> None:
        '''Insert value at index.'''
        if index < 0 or index > self._size:
            raise IndexError('index out of range')
        if index == 0:
            self.prepend(value)
            return
        node = _SNode(value)
        previous = self._node_at(index - 1)
        node.next_node = previous.next_node
        previous.next_node = node
        if not node.next_node:
            self._tail = node
        self._size += 1

    def remove(self, value: T) -> None:
        '''Remove the first occurrence of value.'''
        if not self._head:
            raise ValueError(f'{value!r} not in list')
        if self._head.value == value:
            self._head = self._head.next_node
            if not self._head:
                self._tail = None
            self._size -= 1
            return
        current = self._head
        while current.next_node:
            if current.next_node.value == value:
                current.next_node = current.next_node.next_node
                if not current.next_node:
                    self._tail = current
                self._size -= 1
                return
            current = current.next_node
        raise ValueError(f'{value!r} not in list')

    def pop(self, index: int = -1) -> T:
        '''Remove and return item at index.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty list')
        if index < 0:
            index += self._size
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        if index == 0:
            value = self._head.value
            self._head = self._head.next_node
            if not self._head:
                self._tail = None
            self._size -= 1
            return value
        previous = self._node_at(index - 1)
        node = previous.next_node
        previous.next_node = node.next_node
        if not previous.next_node:
            self._tail = previous
        self._size -= 1
        return node.value

    def find(self, value: T) -> int:
        '''Return the index of the first occurrence of value.'''
        current = self._head
        index = 0
        while current:
            if current.value == value:
                return index
            current = current.next_node
            index += 1
        raise ValueError(f'{value!r} not in list')

    def __getitem__(self, index: int) -> T:
        return self._node_at(index).value

    def __setitem__(self, index: int, value: T) -> None:
        self._node_at(index).value = value

    def __iter__(self) -> Iterator[T]:
        current = self._head
        while current:
            yield current.value
            current = current.next_node

    def __contains__(self, value: T) -> bool:
        try:
            self.find(value)
            return True
        except ValueError:
            return False

    def __repr__(self) -> str:
        return f'SinglyLinkedList({list(self)})'


class _DNode(Generic[T]):
    __slots__ = ('value', 'prev_node', 'next_node')

    def __init__(self, value: T) -> None:
        self.value = value
        self.prev_node: Optional['_DNode[T]'] = None
        self.next_node: Optional['_DNode[T]'] = None


class DoublyLinkedList(Generic[T]):
    '''A doubly linked list.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._head: Optional[_DNode[T]] = None
        self._tail: Optional[_DNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.append(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def _node_at(self, index: int) -> _DNode[T]:
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        if index < self._size // 2:
            current = self._head
            for _ in range(index):
                current = current.next_node
        else:
            current = self._tail
            for _ in range(self._size - 1 - index):
                current = current.prev_node
        return current

    def append(self, value: T) -> None:
        '''Append a value to the end.'''
        node = _DNode(value)
        if not self._tail:
            self._head = self._tail = node
        else:
            node.prev_node = self._tail
            self._tail.next_node = node
            self._tail = node
        self._size += 1

    def prepend(self, value: T) -> None:
        '''Insert a value at the front.'''
        node = _DNode(value)
        if not self._head:
            self._head = self._tail = node
        else:
            node.next_node = self._head
            self._head.prev_node = node
            self._head = node
        self._size += 1

    def insert(self, index: int, value: T) -> None:
        '''Insert value at index.'''
        if index < 0 or index > self._size:
            raise IndexError('index out of range')
        if index == 0:
            self.prepend(value)
            return
        if index == self._size:
            self.append(value)
            return
        next_node = self._node_at(index)
        prev_node = next_node.prev_node
        new_node = _DNode(value)
        new_node.prev_node = prev_node
        new_node.next_node = next_node
        prev_node.next_node = new_node
        next_node.prev_node = new_node
        self._size += 1

    def remove(self, value: T) -> None:
        '''Remove the first occurrence of value.'''
        current = self._head
        while current:
            if current.value == value:
                self._remove_node(current)
                return
            current = current.next_node
        raise ValueError(f'{value!r} not in list')

    def _remove_node(self, node: _DNode[T]) -> None:
        if node.prev_node:
            node.prev_node.next_node = node.next_node
        else:
            self._head = node.next_node
        if node.next_node:
            node.next_node.prev_node = node.prev_node
        else:
            self._tail = node.prev_node
        self._size -= 1

    def pop(self, index: int = -1) -> T:
        '''Remove and return item at index.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty list')
        if index < 0:
            index += self._size
        if not 0 <= index < self._size:
            raise IndexError('index out of range')
        node = self._node_at(index)
        value = node.value
        self._remove_node(node)
        return value

    def find(self, value: T) -> int:
        '''Return the index of the first occurrence of value.'''
        current = self._head
        index = 0
        while current:
            if current.value == value:
                return index
            current = current.next_node
            index += 1
        raise ValueError(f'{value!r} not in list')

    def __getitem__(self, index: int) -> T:
        return self._node_at(index).value

    def __setitem__(self, index: int, value: T) -> None:
        self._node_at(index).value = value

    def __iter__(self) -> Iterator[T]:
        current = self._head
        while current:
            yield current.value
            current = current.next_node

    def __contains__(self, value: T) -> bool:
        try:
            self.find(value)
            return True
        except ValueError:
            return False

    def __repr__(self) -> str:
        return f'DoublyLinkedList({list(self)})'


class _CNode(Generic[T]):
    __slots__ = ('value', 'next_node')

    def __init__(self, value: T) -> None:
        self.value = value
        self.next_node: Optional['_CNode[T]'] = None


class CircularLinkedList(Generic[T]):
    '''A circular linked list.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._tail: Optional[_CNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.append(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def append(self, value: T) -> None:
        '''Append a value.'''
        new_node = _CNode(value)
        if not self._tail:
            new_node.next_node = new_node
            self._tail = new_node
        else:
            new_node.next_node = self._tail.next_node
            self._tail.next_node = new_node
            self._tail = new_node
        self._size += 1

    def prepend(self, value: T) -> None:
        '''Insert a value at the front.'''
        new_node = _CNode(value)
        if not self._tail:
            new_node.next_node = new_node
            self._tail = new_node
        else:
            new_node.next_node = self._tail.next_node
            self._tail.next_node = new_node
        self._size += 1

    def remove(self, value: T) -> None:
        '''Remove the first occurrence of value.'''
        if self.is_empty():
            raise ValueError(f'{value!r} not in list')
        head = self._tail.next_node
        if head.value == value:
            if head is self._tail:
                self._tail = None
            else:
                self._tail.next_node = head.next_node
            self._size -= 1
            return
        current = head
        for _ in range(self._size - 1):
            if current.next_node.value == value:
                current.next_node = current.next_node.next_node
                if current.next_node is head:
                    self._tail = current
                self._size -= 1
                return
            current = current.next_node
        raise ValueError(f'{value!r} not in list')

    def __iter__(self) -> Iterator[T]:
        if not self._tail:
            return
        start = self._tail.next_node
        current = start
        while True:
            yield current.value
            current = current.next_node
            if current is start:
                break

    def _find(self, value: T) -> int:
        '''Return the index of the first occurrence of value.'''
        if not self._tail:
            raise ValueError(f'{value!r} not in list')
        start = self._tail.next_node
        current = start
        index = 0
        while True:
            if current.value == value:
                return index
            current = current.next_node
            index += 1
            if current is start:
                break
        raise ValueError(f'{value!r} not in list')

    def __contains__(self, value: T) -> bool:
        try:
            self._find(value)
            return True
        except ValueError:
            return False

    def __repr__(self) -> str:
        return f'CircularLinkedList({list(self)})'


class Stack(Generic[T]):
    '''A last-in, first-out (LIFO) stack.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._items: List[T] = []
        if iterable is not None:
            for item in iterable:
                self.push(item)

    def __len__(self) -> int:
        return len(self._items)

    def is_empty(self) -> bool:
        return len(self._items) == 0

    def push(self, value: T) -> None:
        '''Push a value onto the stack.'''
        self._items.append(value)

    def pop(self) -> T:
        '''Remove and return the top value.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty stack')
        return self._items.pop()

    def peek(self) -> T:
        '''Return the top value without removing it.'''
        if self.is_empty():
            raise EmptyStructureError('peek from empty stack')
        return self._items[-1]

    def __iter__(self) -> Iterator[T]:
        return iter(reversed(self._items))

    def __contains__(self, value: T) -> bool:
        return value in self._items

    def __repr__(self) -> str:
        return f'Stack({list(reversed(self._items))})'


class _QNode(Generic[T]):
    __slots__ = ('value', 'next_node')

    def __init__(self, value: T) -> None:
        self.value = value
        self.next_node: Optional['_QNode[T]'] = None


class Queue(Generic[T]):
    '''A first-in, first-out (FIFO) queue.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._front: Optional[_QNode[T]] = None
        self._rear: Optional[_QNode[T]] = None
        self._size = 0
        if iterable is not None:
            for item in iterable:
                self.enqueue(item)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def enqueue(self, value: T) -> None:
        '''Add a value to the end of the queue.'''
        node = _QNode(value)
        if not self._rear:
            self._front = self._rear = node
        else:
            self._rear.next_node = node
            self._rear = node
        self._size += 1

    def dequeue(self) -> T:
        '''Remove and return the front value.'''
        if self.is_empty():
            raise EmptyStructureError('dequeue from empty queue')
        value = self._front.value
        self._front = self._front.next_node
        if not self._front:
            self._rear = None
        self._size -= 1
        return value

    def peek(self) -> T:
        '''Return the front value without removing it.'''
        if self.is_empty():
            raise EmptyStructureError('peek from empty queue')
        return self._front.value

    def __iter__(self) -> Iterator[T]:
        current = self._front
        while current:
            yield current.value
            current = current.next_node

    def __contains__(self, value: T) -> bool:
        return value in list(self)

    def __repr__(self) -> str:
        return f'Queue({list(self)})'
