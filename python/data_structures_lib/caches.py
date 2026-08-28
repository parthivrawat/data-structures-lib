'''Cache data structures.'''

from collections import OrderedDict
from typing import Dict, Generic, TypeVar

K = TypeVar('K')
V = TypeVar('V')

__all__ = ['LRUCache']


class LRUCache(Generic[K, V]):
    '''A least-recently-used cache with a fixed capacity.'''

    def __init__(self, capacity: int) -> None:
        if capacity <= 0:
            raise ValueError('capacity must be positive')
        self._capacity = capacity
        self._cache: Dict[K, V] = {}
        self._order: 'OrderedDict[K, V]' = OrderedDict()

    def __len__(self) -> int:
        return len(self._cache)

    def __contains__(self, key: K) -> bool:
        return key in self._cache

    def get(self, key: K) -> V:
        '''Return the value for key, raising KeyError if not found.'''
        if key not in self._cache:
            raise KeyError(key)
        self._order.move_to_end(key)
        return self._cache[key]

    def put(self, key: K, value: V) -> None:
        '''Insert or update a key-value pair.'''
        if key in self._cache:
            self._cache[key] = value
            self._order.move_to_end(key)
        else:
            if len(self._cache) >= self._capacity:
                oldest = next(iter(self._order))
                del self._order[oldest]
                del self._cache[oldest]
            self._cache[key] = value
            self._order[key] = value

    def __getitem__(self, key: K) -> V:
        return self.get(key)

    def __setitem__(self, key: K, value: V) -> None:
        self.put(key, value)

    def __repr__(self) -> str:
        return f'LRUCache({dict(self._cache)})'
