'''Probabilistic data structures.'''

import math
from typing import Any, Iterator

__all__ = ['BloomFilter']


class BloomFilter:
    '''A Bloom filter for membership queries.'''

    def __init__(self, expected_items: int = 1000, false_positive_rate: float = 0.01) -> None:
        if expected_items <= 0:
            raise ValueError('expected_items must be positive')
        if not 0 < false_positive_rate < 1:
            raise ValueError('false_positive_rate must be between 0 and 1')
        n = expected_items
        p = false_positive_rate
        m = math.ceil(-(n * math.log(p)) / (math.log(2) ** 2))
        k = max(1, round((m / n) * math.log(2)))
        self._size = m
        self._hash_count = k
        self._expected_items = n
        self._false_positive_rate = p
        self._bits = bytearray((m + 7) // 8)
        self._items_added = 0

    def _hashes(self, item: Any) -> Iterator[int]:
        h1 = hash(item)
        h2 = hash((item, 7))
        for i in range(self._hash_count):
            yield abs((h1 + i * h2) % self._size)

    def add(self, item: Any) -> None:
        '''Add an item to the filter.'''
        for position in self._hashes(item):
            self._bits[position // 8] |= 1 << (position % 8)
        self._items_added += 1

    def __contains__(self, item: Any) -> bool:
        for position in self._hashes(item):
            if not (self._bits[position // 8] & (1 << (position % 8))):
                return False
        return True

    def __len__(self) -> int:
        return self._items_added

    def expected_fpp(self) -> float:
        '''Return the current estimated false-positive probability.'''
        return (1 - math.exp(-self._hash_count * self._items_added / self._size)) ** self._hash_count

    def __repr__(self) -> str:
        return (
            f'BloomFilter('
            f'size={self._size}, '
            f'hash_count={self._hash_count}, '
            f'items_added={self._items_added})'
        )
