use data_structures_lib::{
    AdjacencyListGraph, AdjacencyMatrixGraph, AvlTree, BinarySearchTree, BloomFilter,
    CircularLinkedList, DoublyLinkedList, DynamicArray, HashMap, HashSet, LruCache,
    MaxHeap, MinHeap, Queue, SinglyLinkedList, Stack, Trie,
};

#[test]
fn test_dynamic_array() {
    let mut arr = DynamicArray::new();
    arr.append(1);
    arr.append(2);
    assert_eq!(arr.len(), 2);
    assert_eq!(arr.get(0), Ok(&1));
    arr.insert(1, 2).unwrap();
    assert_eq!(arr.pop(None), Ok(2));
}

#[test]
fn test_singly_linked_list() {
    let mut list = SinglyLinkedList::new();
    list.append(1);
    list.append(2);
    assert_eq!(list.len(), 2);
    list.remove(&2).unwrap();
    assert_eq!(list.len(), 1);
}

#[test]
fn test_doubly_linked_list() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.prepend(2);
    assert_eq!(list.to_vec(), vec![2, 1]);
    list.remove(&1).unwrap();
    assert_eq!(list.len(), 1);
}

#[test]
fn test_circular_linked_list() {
    let mut list = CircularLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.remove(&2).unwrap();
    assert_eq!(list.to_vec(), vec![1, 3]);
}

#[test]
fn test_stack() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), Ok(2));
    assert_eq!(s.peek(), Ok(&1));
}

#[test]
fn test_queue() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    assert_eq!(q.dequeue(), Ok(1));
    assert_eq!(q.peek(), Ok(&2));
}

#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.set("a", 1);
    map.set("b", 2);
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.add(1);
    set.add(2);
    assert!(set.has(&1));
    set.remove(&2);
    assert!(!set.has(&2));
}

#[test]
fn test_binary_search_tree() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    assert!(tree.search(&3));
    assert_eq!(tree.in_order(), vec![3, 5, 7]);
    tree.delete(&3).unwrap();
    assert!(!tree.search(&3));
}

#[test]
fn test_avl_tree() {
    let mut tree = AvlTree::new();
    for i in 0..100 {
        tree.insert(i);
    }
    assert!(tree.height() < 20);
    tree.delete(&50).unwrap();
    assert!(!tree.search(&50));
}

#[test]
fn test_min_heap() {
    let mut heap = MinHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(3);
    assert_eq!(heap.pop(), Ok(1));
    assert_eq!(heap.pop(), Ok(3));
}

#[test]
fn test_max_heap() {
    let mut heap = MaxHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(3);
    assert_eq!(heap.pop(), Ok(5));
    assert_eq!(heap.pop(), Ok(3));
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("cat");
    trie.insert("car");
    assert!(trie.search("cat"));
    assert!(!trie.search("ca"));
    assert!(trie.starts_with("ca"));
    assert_eq!(trie.words("ca").len(), 2);
}

#[test]
fn test_adjacency_list_graph() {
    let mut g = AdjacencyListGraph::new(false);
    g.add_edge("a", "b");
    g.add_edge("a", "c");
    g.add_edge("b", "d");
    let bfs = g.bfs(&"a");
    assert_eq!(bfs.len(), 4);
}

#[test]
fn test_adjacency_matrix_graph() {
    let mut g = AdjacencyMatrixGraph::new(false);
    g.add_edge(1, 2, 1);
    g.add_edge(1, 3, 1);
    assert_eq!(g.neighbors(&1).len(), 2);
}

#[test]
fn test_lru_cache() {
    let mut cache = LruCache::new(2).unwrap();
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3);
    assert!(!cache.has(&"a"));
    cache.get(&"b").unwrap();
    cache.set("d", 4);
    assert!(!cache.has(&"c"));
}

#[test]
fn test_bloom_filter() {
    let mut bf = BloomFilter::new(100, 0.05).unwrap();
    for i in 0..50 {
        bf.add(&format!("item_{}", i));
    }
    assert!(bf.has("item_0"));
    assert!(bf.has("item_25"));
}
