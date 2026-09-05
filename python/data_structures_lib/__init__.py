'''Data Structures Library for Python.

A zero-dependency collection of fundamental data structures.
'''

from .caches import LRUCache
from .exceptions import EmptyStructureError
from .graphs import AdjacencyListGraph, AdjacencyMatrixGraph
from .hashing import HashMap, HashSet
from .heaps import MaxHeap, MinHeap
from .linear import (
    CircularLinkedList,
    DoublyLinkedList,
    DynamicArray,
    Queue,
    SinglyLinkedList,
    Stack,
)
from .probabilistic import BloomFilter
from .trees import AVLTree, BinarySearchTree
from .tries import Trie

__version__ = '1.1.0'

__all__ = [
    'AVLTree',
    'AdjacencyListGraph',
    'AdjacencyMatrixGraph',
    'BinarySearchTree',
    'BloomFilter',
    'CircularLinkedList',
    'DoublyLinkedList',
    'DynamicArray',
    'EmptyStructureError',
    'HashMap',
    'HashSet',
    'LRUCache',
    'MaxHeap',
    'MinHeap',
    'Queue',
    'SinglyLinkedList',
    'Stack',
    'Trie',
]
