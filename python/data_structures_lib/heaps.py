'''Heap-based data structures.'''

from typing import Generic, Iterable, Iterator, List, Optional, TypeVar

from .exceptions import EmptyStructureError

T = TypeVar('T')

__all__ = ['MinHeap', 'MaxHeap']


class MinHeap(Generic[T]):
    '''A min-heap.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._data: List[T] = []
        if iterable is not None:
            for item in iterable:
                self.push(item)

    def __len__(self) -> int:
        return len(self._data)

    def is_empty(self) -> bool:
        return len(self._data) == 0

    def push(self, value: T) -> None:
        '''Add a value.'''
        self._data.append(value)
        self._sift_up(len(self._data) - 1)

    def _sift_up(self, index: int) -> None:
        parent = (index - 1) // 2
        while index > 0 and self._data[index] < self._data[parent]:
            self._data[index], self._data[parent] = self._data[parent], self._data[index]
            index = parent
            parent = (index - 1) // 2

    def pop(self) -> T:
        '''Remove and return the smallest value.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty heap')
        if len(self._data) == 1:
            return self._data.pop()
        root = self._data[0]
        self._data[0] = self._data.pop()
        self._sift_down(0)
        return root

    def _sift_down(self, index: int) -> None:
        length = len(self._data)
        while True:
            smallest = index
            left = 2 * index + 1
            right = 2 * index + 2
            if left < length and self._data[left] < self._data[smallest]:
                smallest = left
            if right < length and self._data[right] < self._data[smallest]:
                smallest = right
            if smallest == index:
                break
            self._data[index], self._data[smallest] = self._data[smallest], self._data[index]
            index = smallest

    def peek(self) -> T:
        '''Return the smallest value without removing it.'''
        if self.is_empty():
            raise EmptyStructureError('peek from empty heap')
        return self._data[0]

    def __iter__(self) -> Iterator[T]:
        return iter(self._data)

    def __contains__(self, value: T) -> bool:
        return value in self._data

    def __repr__(self) -> str:
        return f'MinHeap({list(self._data)})'


class MaxHeap(Generic[T]):
    '''A max-heap.'''

    def __init__(self, iterable: Optional[Iterable[T]] = None) -> None:
        self._data: List[T] = []
        if iterable is not None:
            for item in iterable:
                self.push(item)

    def __len__(self) -> int:
        return len(self._data)

    def is_empty(self) -> bool:
        return len(self._data) == 0

    def push(self, value: T) -> None:
        '''Add a value.'''
        self._data.append(value)
        self._sift_up(len(self._data) - 1)

    def _sift_up(self, index: int) -> None:
        parent = (index - 1) // 2
        while index > 0 and self._data[index] > self._data[parent]:
            self._data[index], self._data[parent] = self._data[parent], self._data[index]
            index = parent
            parent = (index - 1) // 2

    def pop(self) -> T:
        '''Remove and return the largest value.'''
        if self.is_empty():
            raise EmptyStructureError('pop from empty heap')
        if len(self._data) == 1:
            return self._data.pop()
        root = self._data[0]
        self._data[0] = self._data.pop()
        self._sift_down(0)
        return root

    def _sift_down(self, index: int) -> None:
        length = len(self._data)
        while True:
            largest = index
            left = 2 * index + 1
            right = 2 * index + 2
            if left < length and self._data[left] > self._data[largest]:
                largest = left
            if right < length and self._data[right] > self._data[largest]:
                largest = right
            if largest == index:
                break
            self._data[index], self._data[largest] = self._data[largest], self._data[index]
            index = largest

    def peek(self) -> T:
        '''Return the largest value without removing it.'''
        if self.is_empty():
            raise EmptyStructureError('peek from empty heap')
        return self._data[0]

    def __iter__(self) -> Iterator[T]:
        return iter(self._data)

    def __contains__(self, value: T) -> bool:
        return value in self._data

    def __repr__(self) -> str:
        return f'MaxHeap({list(self._data)})'
