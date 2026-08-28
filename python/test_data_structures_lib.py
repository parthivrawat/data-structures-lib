'''Tests for the data structures library.'''

import pytest

from data_structures_lib import (
    AVLTree,
    AdjacencyListGraph,
    AdjacencyMatrixGraph,
    BinarySearchTree,
    BloomFilter,
    CircularLinkedList,
    DoublyLinkedList,
    DynamicArray,
    EmptyStructureError,
    HashMap,
    HashSet,
    LRUCache,
    MaxHeap,
    MinHeap,
    Queue,
    SinglyLinkedList,
    Stack,
    Trie,
)


class TestDynamicArray:
    def test_append_and_get(self):
        arr = DynamicArray()
        arr.append(1)
        arr.append(2)
        assert len(arr) == 2
        assert arr[0] == 1
        assert arr[1] == 2

    def test_insert(self):
        arr = DynamicArray()
        arr.append(1)
        arr.append(3)
        arr.insert(1, 2)
        assert list(arr) == [1, 2, 3]

    def test_pop(self):
        arr = DynamicArray()
        arr.append(1)
        arr.append(2)
        assert arr.pop() == 2
        assert arr.pop(0) == 1

    def test_remove(self):
        arr = DynamicArray()
        arr.append(1)
        arr.append(2)
        arr.remove(1)
        assert list(arr) == [2]

    def test_resize(self):
        arr = DynamicArray(2)
        for i in range(10):
            arr.append(i)
        assert arr.capacity >= 10


class TestSinglyLinkedList:
    def test_append_and_get(self):
        ll = SinglyLinkedList()
        ll.append(1)
        ll.append(2)
        assert len(ll) == 2
        assert ll[0] == 1
        assert ll[1] == 2

    def test_prepend(self):
        ll = SinglyLinkedList()
        ll.prepend(1)
        ll.prepend(2)
        assert list(ll) == [2, 1]

    def test_remove(self):
        ll = SinglyLinkedList([1, 2, 3])
        ll.remove(2)
        assert list(ll) == [1, 3]

    def test_pop(self):
        ll = SinglyLinkedList([1, 2, 3])
        assert ll.pop() == 3
        assert ll.pop(0) == 1


class TestDoublyLinkedList:
    def test_append_and_get(self):
        dll = DoublyLinkedList()
        dll.append(1)
        dll.append(2)
        assert len(dll) == 2
        assert dll[0] == 1

    def test_prepend(self):
        dll = DoublyLinkedList()
        dll.prepend(1)
        dll.prepend(2)
        assert list(dll) == [2, 1]

    def test_remove(self):
        dll = DoublyLinkedList([1, 2, 3])
        dll.remove(2)
        assert list(dll) == [1, 3]

    def test_pop(self):
        dll = DoublyLinkedList([1, 2, 3])
        assert dll.pop() == 3
        assert dll.pop(0) == 1


class TestCircularLinkedList:
    def test_append(self):
        cll = CircularLinkedList()
        cll.append(1)
        cll.append(2)
        assert len(cll) == 2
        assert list(cll) == [1, 2]

    def test_prepend(self):
        cll = CircularLinkedList()
        cll.append(2)
        cll.prepend(1)
        assert list(cll) == [1, 2]

    def test_remove(self):
        cll = CircularLinkedList([1, 2, 3])
        cll.remove(2)
        assert list(cll) == [1, 3]


class TestStack:
    def test_push_pop(self):
        s = Stack()
        s.push(1)
        s.push(2)
        assert s.pop() == 2
        assert s.pop() == 1

    def test_peek(self):
        s = Stack([1, 2])
        assert s.peek() == 2
        assert len(s) == 2

    def test_empty_pop(self):
        s = Stack()
        with pytest.raises(EmptyStructureError):
            s.pop()


class TestQueue:
    def test_enqueue_dequeue(self):
        q = Queue()
        q.enqueue(1)
        q.enqueue(2)
        assert q.dequeue() == 1
        assert q.dequeue() == 2

    def test_peek(self):
        q = Queue([1, 2])
        assert q.peek() == 1
        assert len(q) == 2

    def test_empty_dequeue(self):
        q = Queue()
        with pytest.raises(EmptyStructureError):
            q.dequeue()


class TestHashMap:
    def test_set_get(self):
        m = HashMap()
        m['a'] = 1
        m['b'] = 2
        assert m['a'] == 1
        assert m['b'] == 2

    def test_delete(self):
        m = HashMap()
        m['a'] = 1
        del m['a']
        assert 'a' not in m

    def test_resize(self):
        m = HashMap(capacity=2)
        for i in range(100):
            m[i] = i
        for i in range(100):
            assert m[i] == i

    def test_get_default(self):
        m = HashMap()
        assert m.get('missing', 0) == 0


class TestHashSet:
    def test_add_contains(self):
        s = HashSet()
        s.add(1)
        s.add(2)
        assert 1 in s
        assert 3 not in s

    def test_remove(self):
        s = HashSet([1, 2, 3])
        s.remove(2)
        assert 2 not in s

    def test_iter(self):
        s = HashSet([1, 2, 3])
        assert len(list(s)) == 3


class TestBinarySearchTree:
    def test_insert_and_contains(self):
        bst = BinarySearchTree()
        bst.insert(5)
        bst.insert(3)
        bst.insert(7)
        assert 3 in bst
        assert 7 in bst
        assert 9 not in bst

    def test_delete(self):
        bst = BinarySearchTree([5, 3, 7, 1, 4])
        bst.delete(3)
        assert 3 not in bst
        assert sorted(list(bst)) == [1, 4, 5, 7]

    def test_inorder(self):
        bst = BinarySearchTree([5, 3, 7, 1, 4])
        assert list(bst) == [1, 3, 4, 5, 7]


class TestAVLTree:
    def test_insert_and_contains(self):
        avl = AVLTree([10, 20, 30, 40, 50])
        for v in [10, 20, 30, 40, 50]:
            assert v in avl

    def test_delete(self):
        avl = AVLTree([10, 20, 30, 40, 50])
        avl.delete(20)
        assert 20 not in avl

    def test_height(self):
        avl = AVLTree(list(range(100)))
        assert avl.height() < 20

    def test_inorder(self):
        avl = AVLTree([3, 1, 2])
        assert list(avl) == [1, 2, 3]


class TestMinHeap:
    def test_pop_order(self):
        h = MinHeap([5, 1, 3, 2, 4])
        assert h.pop() == 1
        assert h.pop() == 2
        assert h.pop() == 3

    def test_peek(self):
        h = MinHeap([5, 1, 3])
        assert h.peek() == 1
        assert len(h) == 3


class TestMaxHeap:
    def test_pop_order(self):
        h = MaxHeap([5, 1, 3, 2, 4])
        assert h.pop() == 5
        assert h.pop() == 4
        assert h.pop() == 3


class TestTrie:
    def test_insert_and_search(self):
        t = Trie()
        t.insert('cat')
        t.insert('car')
        assert t.search('cat')
        assert t.search('car')
        assert not t.search('ca')

    def test_starts_with(self):
        t = Trie()
        t.insert('cat')
        assert t.starts_with('ca')
        assert not t.starts_with('dog')

    def test_delete(self):
        t = Trie(['cat', 'car'])
        t.delete('cat')
        assert not t.search('cat')
        assert t.search('car')

    def test_words(self):
        t = Trie(['cat', 'car', 'card'])
        assert t.words('ca') == ['cat', 'car', 'card']


class TestAdjacencyListGraph:
    def test_bfs(self):
        g = AdjacencyListGraph()
        g.add_edge('a', 'b')
        g.add_edge('a', 'c')
        g.add_edge('b', 'd')
        assert g.bfs('a') == ['a', 'b', 'c', 'd']

    def test_dfs(self):
        g = AdjacencyListGraph()
        g.add_edge('a', 'b')
        g.add_edge('a', 'c')
        g.add_edge('b', 'd')
        result = g.dfs('a')
        assert result[0] == 'a'
        assert set(result) == {'a', 'b', 'c', 'd'}


class TestAdjacencyMatrixGraph:
    def test_add_edge_and_neighbors(self):
        g = AdjacencyMatrixGraph()
        g.add_edge(1, 2)
        g.add_edge(1, 3)
        assert set(g.neighbors(1)) == {2, 3}

    def test_remove_vertex(self):
        g = AdjacencyMatrixGraph()
        g.add_edge(1, 2)
        g.remove_vertex(2)
        assert 2 not in g


class TestLRUCache:
    def test_basic(self):
        cache = LRUCache(capacity=2)
        cache['a'] = 1
        cache['b'] = 2
        cache['c'] = 3
        assert 'a' not in cache
        assert 'b' in cache
        assert 'c' in cache

    def test_update_marks_recent(self):
        cache = LRUCache(capacity=2)
        cache['a'] = 1
        cache['b'] = 2
        _ = cache['a']
        cache['c'] = 3
        assert 'a' in cache
        assert 'b' not in cache


class TestBloomFilter:
    def test_membership(self):
        bf = BloomFilter(expected_items=100, false_positive_rate=0.05)
        for i in range(50):
            bf.add(f'item_{i}')
        for i in range(50):
            assert f'item_{i}' in bf

    def test_false_negatives_impossible(self):
        bf = BloomFilter(expected_items=100, false_positive_rate=0.1)
        bf.add('hello')
        assert 'hello' in bf


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
