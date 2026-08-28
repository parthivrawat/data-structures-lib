import { describe, it, expect } from 'vitest';
import {
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
} from './index';

describe('DynamicArray', () => {
  it('appends and gets', () => {
    const arr = new DynamicArray<number>();
    arr.append(1);
    arr.append(2);
    expect(arr.length).toBe(2);
    expect(arr.get(0)).toBe(1);
  });

  it('inserts', () => {
    const arr = new DynamicArray<number>();
    arr.append(1);
    arr.append(3);
    arr.insert(1, 2);
    expect(Array.from(arr)).toEqual([1, 2, 3]);
  });

  it('pops', () => {
    const arr = new DynamicArray<number>();
    arr.append(1);
    arr.append(2);
    expect(arr.pop()).toBe(2);
    expect(arr.pop(0)).toBe(1);
  });
});

describe('SinglyLinkedList', () => {
  it('appends and gets', () => {
    const list = new SinglyLinkedList<number>();
    list.append(1);
    list.append(2);
    expect(list.length).toBe(2);
    expect(list.get(0)).toBe(1);
  });

  it('removes', () => {
    const list = new SinglyLinkedList([1, 2, 3]);
    list.remove(2);
    expect(Array.from(list)).toEqual([1, 3]);
  });
});

describe('DoublyLinkedList', () => {
  it('appends and prepends', () => {
    const list = new DoublyLinkedList<number>();
    list.append(1);
    list.prepend(2);
    expect(Array.from(list)).toEqual([2, 1]);
  });

  it('removes', () => {
    const list = new DoublyLinkedList([1, 2, 3]);
    list.remove(2);
    expect(Array.from(list)).toEqual([1, 3]);
  });
});

describe('CircularLinkedList', () => {
  it('appends and removes', () => {
    const list = new CircularLinkedList([1, 2, 3]);
    list.remove(2);
    expect(Array.from(list)).toEqual([1, 3]);
  });

  it('prepends', () => {
    const list = new CircularLinkedList<number>();
    list.append(2);
    list.prepend(1);
    expect(Array.from(list)).toEqual([1, 2]);
  });
});

describe('Stack', () => {
  it('pushes and pops', () => {
    const stack = new Stack<number>();
    stack.push(1);
    stack.push(2);
    expect(stack.pop()).toBe(2);
    expect(stack.pop()).toBe(1);
  });

  it('throws on empty pop', () => {
    const stack = new Stack<number>();
    expect(() => stack.pop()).toThrow(EmptyStructureError);
  });
});

describe('Queue', () => {
  it('enqueues and dequeues', () => {
    const q = new Queue<number>();
    q.enqueue(1);
    q.enqueue(2);
    expect(q.dequeue()).toBe(1);
    expect(q.dequeue()).toBe(2);
  });

  it('throws on empty dequeue', () => {
    const q = new Queue<number>();
    expect(() => q.dequeue()).toThrow(EmptyStructureError);
  });
});

describe('HashMap', () => {
  it('sets and gets', () => {
    const map = new HashMap<string, number>();
    map.set('a', 1);
    map.set('b', 2);
    expect(map.get('a')).toBe(1);
    expect(map.get('missing')).toBeUndefined();
  });

  it('resizes', () => {
    const map = new HashMap<number, number>(2);
    for (let i = 0; i < 100; i++) {
      map.set(i, i);
    }
    for (let i = 0; i < 100; i++) {
      expect(map.get(i)).toBe(i);
    }
  });
});

describe('HashSet', () => {
  it('adds and checks', () => {
    const set = new HashSet<number>();
    set.add(1);
    set.add(2);
    expect(set.has(1)).toBe(true);
    expect(set.has(3)).toBe(false);
  });

  it('removes', () => {
    const set = new HashSet([1, 2, 3]);
    set.remove(2);
    expect(set.has(2)).toBe(false);
  });
});

describe('BinarySearchTree', () => {
  it('inserts and searches', () => {
    const tree = new BinarySearchTree<number>();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    expect(tree.has(3)).toBe(true);
    expect(tree.has(9)).toBe(false);
  });

  it('deletes', () => {
    const tree = new BinarySearchTree<number>([5, 3, 7, 1, 4]);
    tree.delete(3);
    expect(tree.has(3)).toBe(false);
  });

  it('iterates in order', () => {
    const tree = new BinarySearchTree<number>([5, 3, 7, 1, 4]);
    expect(Array.from(tree)).toEqual([1, 3, 4, 5, 7]);
  });
});

describe('AVLTree', () => {
  it('inserts and searches', () => {
    const tree = new AVLTree<number>([10, 20, 30, 40, 50]);
    for (const v of [10, 20, 30, 40, 50]) {
      expect(tree.has(v)).toBe(true);
    }
  });

  it('deletes', () => {
    const tree = new AVLTree<number>([10, 20, 30, 40, 50]);
    tree.delete(20);
    expect(tree.has(20)).toBe(false);
  });

  it('stays balanced', () => {
    const tree = new AVLTree<number>([...Array(100).keys()]);
    expect(tree.height()).toBeLessThan(20);
  });
});

describe('MinHeap', () => {
  it('pops in order', () => {
    const heap = new MinHeap<number>([5, 1, 3, 2, 4]);
    expect(heap.pop()).toBe(1);
    expect(heap.pop()).toBe(2);
    expect(heap.pop()).toBe(3);
  });
});

describe('MaxHeap', () => {
  it('pops in order', () => {
    const heap = new MaxHeap<number>([5, 1, 3, 2, 4]);
    expect(heap.pop()).toBe(5);
    expect(heap.pop()).toBe(4);
    expect(heap.pop()).toBe(3);
  });
});

describe('Trie', () => {
  it('inserts and searches', () => {
    const trie = new Trie();
    trie.insert('cat');
    trie.insert('car');
    expect(trie.search('cat')).toBe(true);
    expect(trie.search('ca')).toBe(false);
  });

  it('checks prefixes', () => {
    const trie = new Trie();
    trie.insert('cat');
    expect(trie.startsWith('ca')).toBe(true);
    expect(trie.startsWith('do')).toBe(false);
  });

  it('deletes', () => {
    const trie = new Trie(['cat', 'car']);
    trie.delete('cat');
    expect(trie.search('cat')).toBe(false);
    expect(trie.search('car')).toBe(true);
  });
});

describe('AdjacencyListGraph', () => {
  it('bfs', () => {
    const g = new AdjacencyListGraph<string>();
    g.addEdge('a', 'b');
    g.addEdge('a', 'c');
    g.addEdge('b', 'd');
    expect(g.bfs('a')).toEqual(['a', 'b', 'c', 'd']);
  });

  it('dfs visits all', () => {
    const g = new AdjacencyListGraph<string>();
    g.addEdge('a', 'b');
    g.addEdge('a', 'c');
    g.addEdge('b', 'd');
    const result = g.dfs('a');
    expect(new Set(result)).toEqual(new Set(['a', 'b', 'c', 'd']));
  });
});

describe('AdjacencyMatrixGraph', () => {
  it('adds edges and returns neighbors', () => {
    const g = new AdjacencyMatrixGraph<number>();
    g.addEdge(1, 2);
    g.addEdge(1, 3);
    expect(new Set(g.neighbors(1))).toEqual(new Set([2, 3]));
  });
});

describe('LRUCache', () => {
  it('evicts least recently used', () => {
    const cache = new LRUCache<string, number>(2);
    cache.set('a', 1);
    cache.set('b', 2);
    cache.set('c', 3);
    expect(cache.has('a')).toBe(false);
    expect(cache.has('b')).toBe(true);
    expect(cache.has('c')).toBe(true);
  });

  it('updates mark as recent', () => {
    const cache = new LRUCache<string, number>(2);
    cache.set('a', 1);
    cache.set('b', 2);
    cache.get('a');
    cache.set('c', 3);
    expect(cache.has('a')).toBe(true);
    expect(cache.has('b')).toBe(false);
  });
});

describe('BloomFilter', () => {
  it('reports added items', () => {
    const bf = new BloomFilter(100, 0.05);
    for (let i = 0; i < 50; i++) {
      bf.add(`item_${i}`);
    }
    for (let i = 0; i < 50; i++) {
      expect(bf.has(`item_${i}`)).toBe(true);
    }
  });
});
