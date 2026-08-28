'''Hashing-based data structures.'''

from typing import Any, Generic, Iterable, Iterator, List, Optional, Tuple, TypeVar

K = TypeVar('K')
V = TypeVar('V')

__all__ = ['HashMap', 'HashSet']


class HashMap(Generic[K, V]):
    '''A hash map using separate chaining.'''

    def __init__(self, capacity: int = 16, load_factor: float = 0.75) -> None:
        if capacity <= 0:
            raise ValueError('capacity must be positive')
        if not 0 < load_factor <= 1:
            raise ValueError('load_factor must be between 0 and 1')
        self._capacity = capacity
        self._load_factor = load_factor
        self._size = 0
        self._table: List[List[Tuple[K, V]]] = [[] for _ in range(capacity)]

    def _index(self, key: K) -> int:
        return hash(key) % self._capacity

    def _resize(self) -> None:
        old_table = self._table
        self._capacity *= 2
        self._size = 0
        self._table = [[] for _ in range(self._capacity)]
        for bucket in old_table:
            for key, value in bucket:
                self._set(key, value, resizing=True)

    def _set(self, key: K, value: V, resizing: bool = False) -> None:
        bucket = self._table[self._index(key)]
        for i, (k, _) in enumerate(bucket):
            if k == key:
                bucket[i] = (key, value)
                return
        bucket.append((key, value))
        if not resizing:
            self._size += 1

    def __setitem__(self, key: K, value: V) -> None:
        self._set(key, value)
        if self._size / self._capacity > self._load_factor:
            self._resize()

    def __getitem__(self, key: K) -> V:
        bucket = self._table[self._index(key)]
        for k, v in bucket:
            if k == key:
                return v
        raise KeyError(key)

    def __delitem__(self, key: K) -> None:
        bucket = self._table[self._index(key)]
        for i, (k, _) in enumerate(bucket):
            if k == key:
                bucket.pop(i)
                self._size -= 1
                return
        raise KeyError(key)

    def __contains__(self, key: K) -> bool:
        try:
            self[key]
            return True
        except KeyError:
            return False

    def __iter__(self) -> Iterator[K]:
        for bucket in self._table:
            for key, _ in bucket:
                yield key

    def keys(self) -> List[K]:
        '''Return a list of keys.'''
        return list(self)

    def values(self) -> List[V]:
        '''Return a list of values.'''
        return [value for bucket in self._table for _, value in bucket]

    def items(self) -> List[Tuple[K, V]]:
        '''Return a list of key-value pairs.'''
        return [(key, value) for bucket in self._table for key, value in bucket]

    def get(self, key: K, default: Optional[V] = None) -> Optional[V]:
        '''Return the value for key, or default.'''
        try:
            return self[key]
        except KeyError:
            return default

    def __len__(self) -> int:
        return self._size

    def __repr__(self) -> str:
        return f'HashMap({dict(self.items())})'


class HashSet(Generic[K]):
    '''A set implemented with a hash map.'''

    def __init__(self, iterable: Optional[Iterable[K]] = None) -> None:
        self._map: HashMap[K, bool] = HashMap()
        if iterable is not None:
            for item in iterable:
                self.add(item)

    def add(self, key: K) -> None:
        '''Add a key to the set.'''
        self._map[key] = True

    def remove(self, key: K) -> None:
        '''Remove a key, raising KeyError if not present.'''
        if key not in self:
            raise KeyError(key)
        del self._map[key]

    def discard(self, key: K) -> None:
        '''Remove a key if present.'''
        if key in self:
            del self._map[key]

    def __contains__(self, key: K) -> bool:
        return key in self._map

    def __iter__(self) -> Iterator[K]:
        return iter(self._map)

    def __len__(self) -> int:
        return len(self._map)

    def __repr__(self) -> str:
        return f'HashSet({list(self)})'
