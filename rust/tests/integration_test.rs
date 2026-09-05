use data_structures_lib::{
    AdjacencyListGraph, AdjacencyMatrixGraph, AvlTree, BinarySearchTree, BloomFilter,
    CircularLinkedList, DoublyLinkedList, DynamicArray, Error, HashMap, HashSet, LruCache,
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

// ---------------------------------------------------------------------------
// Edge cases and regression tests
// ---------------------------------------------------------------------------

// -- DynamicArray --

#[test]
fn test_dynamic_array_empty_pop() {
    let mut arr: DynamicArray<i32> = DynamicArray::new();
    assert_eq!(arr.pop(None), Err(Error::Empty));
    assert_eq!(arr.pop(Some(0)), Err(Error::Empty));
}

#[test]
fn test_dynamic_array_get_out_of_bounds() {
    let mut arr = DynamicArray::new();
    arr.append(1);
    assert_eq!(arr.get(1), Err(Error::OutOfBounds));
    assert_eq!(arr.get(100), Err(Error::OutOfBounds));
    assert_eq!(arr.pop(Some(5)), Err(Error::OutOfBounds));
    assert_eq!(arr.set(3, 9), Err(Error::OutOfBounds));
}

#[test]
fn test_dynamic_array_remove_not_found() {
    let mut arr = DynamicArray::new();
    arr.append(1);
    assert_eq!(arr.remove(&42), Err(Error::NotFound));
    let mut empty: DynamicArray<i32> = DynamicArray::new();
    assert_eq!(empty.remove(&1), Err(Error::NotFound));
}

#[test]
fn test_dynamic_array_append_pop_append_order() {
    let mut arr = DynamicArray::new();
    for _ in 0..3 {
        arr.append(1);
        arr.append(2);
        assert_eq!(arr.pop(None), Ok(2));
        arr.append(3);
        assert_eq!(arr.to_vec(), vec![1, 3]);
        arr.pop(None).unwrap();
        arr.pop(None).unwrap();
    }
    assert!(arr.is_empty());
}

#[test]
fn test_dynamic_array_insert_at_end_and_middle() {
    let mut arr = DynamicArray::new();
    arr.append(1);
    arr.append(3);
    arr.insert(2, 4).unwrap(); // at end
    arr.insert(1, 2).unwrap(); // middle
    assert_eq!(arr.to_vec(), vec![1, 2, 3, 4]);
    assert_eq!(arr.insert(10, 9), Err(Error::OutOfBounds));
}

// -- SinglyLinkedList --

#[test]
fn test_singly_linked_list_empty_pop() {
    let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(list.pop(None), Err(Error::Empty));
    assert_eq!(list.pop(Some(0)), Err(Error::Empty));
}

#[test]
fn test_singly_linked_list_remove_not_found() {
    let mut list = SinglyLinkedList::new();
    assert_eq!(list.remove(&1), Err(Error::NotFound));
    list.append(1);
    assert_eq!(list.remove(&2), Err(Error::NotFound));
    assert_eq!(list.find(&2), Err(Error::NotFound));
}

#[test]
fn test_singly_linked_list_pop_out_of_range() {
    let mut list = SinglyLinkedList::new();
    list.append(1);
    list.append(2);
    assert_eq!(list.pop(Some(2)), Err(Error::OutOfBounds));
    assert_eq!(list.pop(Some(99)), Err(Error::OutOfBounds));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_singly_linked_list_duplicate_values() {
    let mut list = SinglyLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(1);
    list.remove(&1).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list.to_vec(), vec![2, 1]);
}

#[test]
fn test_singly_linked_list_insert_remove_cycles() {
    let mut list = SinglyLinkedList::new();
    for i in 0..5 {
        list.append(i);
        list.prepend(-i);
        list.insert(1, 100 + i).unwrap();
        list.remove(&(100 + i)).unwrap();
        assert_eq!(list.pop(None), Ok(i));
        assert_eq!(list.pop(Some(0)), Ok(-i));
        assert!(list.is_empty());
    }
    assert_eq!(list.insert(1, 0), Err(Error::OutOfBounds));
}

#[test]
fn test_singly_linked_list_string_values() {
    let mut list = SinglyLinkedList::new();
    list.append(String::from("a"));
    list.append(String::from("b"));
    list.append(String::from("c"));
    assert_eq!(list.pop(Some(1)), Ok(String::from("b")));
    assert_eq!(list.to_vec(), vec![String::from("a"), String::from("c")]);
    list.remove(&String::from("a")).unwrap();
    assert_eq!(list.pop(None), Ok(String::from("c")));
    assert!(list.is_empty());
}

#[test]
fn test_singly_linked_list_boxed_values() {
    let mut list = SinglyLinkedList::new();
    list.append(Box::new(1u8));
    list.append(Box::new(2u8));
    assert_eq!(list.pop(None), Ok(Box::new(2u8)));
    assert_eq!(list.len(), 1);
}

// -- DoublyLinkedList --

#[test]
fn test_doubly_linked_list_empty_pop() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    assert_eq!(list.pop(None), Err(Error::Empty));
    assert_eq!(list.pop(Some(0)), Err(Error::Empty));
}

#[test]
fn test_doubly_linked_list_remove_not_found() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.remove(&1), Err(Error::NotFound));
    list.append(1);
    assert_eq!(list.remove(&2), Err(Error::NotFound));
}

#[test]
fn test_doubly_linked_list_pop_out_of_range() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    assert_eq!(list.pop(Some(5)), Err(Error::OutOfBounds));
    assert_eq!(list.len(), 1);
}

#[test]
fn test_doubly_linked_list_duplicate_values() {
    let mut list = DoublyLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(1);
    list.remove(&1).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list.to_vec(), vec![2, 1]);
}

#[test]
fn test_doubly_linked_list_insert_remove_cycles() {
    let mut list = DoublyLinkedList::new();
    for i in 0..5 {
        list.append(i);
        list.prepend(-i);
        list.insert(1, 100 + i).unwrap();
        list.remove(&(100 + i)).unwrap();
        assert_eq!(list.pop(None), Ok(i));
        assert_eq!(list.pop(Some(0)), Ok(-i));
        assert!(list.is_empty());
    }
    assert_eq!(list.insert(1, 0), Err(Error::OutOfBounds));
}

#[test]
fn test_doubly_linked_list_string_values() {
    let mut list = DoublyLinkedList::new();
    list.append(String::from("x"));
    list.prepend(String::from("y"));
    assert_eq!(list.pop(Some(0)), Ok(String::from("y")));
    list.remove(&String::from("x")).unwrap();
    assert!(list.is_empty());
    let mut list: DoublyLinkedList<Box<u8>> = DoublyLinkedList::new();
    list.append(Box::new(7u8));
    assert_eq!(list.pop(None), Ok(Box::new(7u8)));
}

// -- CircularLinkedList --

#[test]
fn test_circular_linked_list_empty_remove() {
    let mut list: CircularLinkedList<i32> = CircularLinkedList::new();
    assert_eq!(list.remove(&1), Err(Error::NotFound));
    assert!(list.is_empty());
    assert_eq!(list.to_vec(), Vec::<i32>::new());
}

#[test]
fn test_circular_linked_list_remove_not_found() {
    let mut list = CircularLinkedList::new();
    list.append(1);
    list.append(2);
    assert_eq!(list.remove(&99), Err(Error::NotFound));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_circular_linked_list_duplicate_values() {
    let mut list = CircularLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(1);
    list.remove(&1).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list.to_vec(), vec![2, 1]);
}

#[test]
fn test_circular_linked_list_repeated_cycles() {
    let mut list = CircularLinkedList::new();
    for i in 0..5 {
        list.append(i);
        list.prepend(-i);
        assert_eq!(list.to_vec(), vec![-i, i]);
        list.remove(&i).unwrap();
        list.remove(&-i).unwrap();
        assert!(list.is_empty());
    }
    // Removing tail and head of a longer list keeps circularity intact.
    for i in 0..4 {
        list.append(i);
    }
    list.remove(&0).unwrap(); // head
    list.remove(&3).unwrap(); // tail
    assert_eq!(list.to_vec(), vec![1, 2]);
    list.remove(&1).unwrap();
    list.remove(&2).unwrap();
    assert!(list.is_empty());
    assert_eq!(list.to_vec(), Vec::<i32>::new());
}

#[test]
fn test_circular_linked_list_string_values() {
    let mut list = CircularLinkedList::new();
    list.append(String::from("a"));
    list.append(String::from("b"));
    list.remove(&String::from("a")).unwrap();
    assert_eq!(list.to_vec(), vec![String::from("b")]);
    let mut list: CircularLinkedList<Box<u8>> = CircularLinkedList::new();
    list.append(Box::new(3u8));
    list.remove(&Box::new(3u8)).unwrap();
    assert!(list.is_empty());
}

// -- Stack --

#[test]
fn test_stack_empty_pop_peek() {
    let mut s: Stack<i32> = Stack::new();
    assert_eq!(s.pop(), Err(Error::Empty));
    assert_eq!(s.peek(), Err(Error::Empty));
}

#[test]
fn test_stack_push_pop_push() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), Ok(2));
    s.push(3);
    assert_eq!(s.peek(), Ok(&3));
    assert_eq!(s.pop(), Ok(3));
    assert_eq!(s.pop(), Ok(1));
    assert_eq!(s.pop(), Err(Error::Empty));
    // Stack is reusable after being drained.
    s.push(9);
    assert_eq!(s.peek(), Ok(&9));
    assert_eq!(s.len(), 1);
}

// -- Queue --

#[test]
fn test_queue_empty_dequeue_peek() {
    let mut q: Queue<i32> = Queue::new();
    assert_eq!(q.dequeue(), Err(Error::Empty));
    assert_eq!(q.peek(), Err(Error::Empty));
}

#[test]
fn test_queue_enqueue_dequeue_enqueue() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    assert_eq!(q.dequeue(), Ok(1));
    q.enqueue(3);
    assert_eq!(q.dequeue(), Ok(2));
    assert_eq!(q.peek(), Ok(&3));
    assert_eq!(q.dequeue(), Ok(3));
    assert_eq!(q.dequeue(), Err(Error::Empty));
    // Queue is reusable after being drained.
    q.enqueue(7);
    assert_eq!(q.len(), 1);
    assert_eq!(q.peek(), Ok(&7));
}

// -- HashMap --

#[test]
fn test_hash_map_missing_key() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.get(&"nope"), None);
    assert_eq!(map.delete(&"nope"), None);
    map.set("a", 1);
    assert_eq!(map.get(&"b"), None);
    assert_eq!(map.delete(&"b"), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_hash_map_overwrite() {
    let mut map = HashMap::new();
    map.set("k", 1);
    map.set("k", 2);
    assert_eq!(map.get(&"k"), Some(&2));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_hash_map_set_delete_cycles() {
    let mut map = HashMap::new();
    for i in 0..10 {
        map.set(i, i * 10);
        assert_eq!(map.get(&i), Some(&(i * 10)));
        assert_eq!(map.delete(&i), Some(i * 10));
        assert!(!map.has(&i));
        assert!(map.is_empty());
    }
}

#[test]
fn test_hash_map_many_items() {
    // Insert enough items to force the underlying map to grow repeatedly.
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.set(i, i);
    }
    assert_eq!(map.len(), 1000);
    for i in 0..1000 {
        assert_eq!(map.get(&i), Some(&i));
    }
}

// -- HashSet --

#[test]
fn test_hash_set_add_duplicate() {
    let mut set = HashSet::new();
    set.add(1);
    set.add(1);
    set.add(1);
    assert_eq!(set.len(), 1);
    assert!(set.has(&1));
}

#[test]
fn test_hash_set_remove_nonexistent() {
    let mut set = HashSet::new();
    assert!(!set.remove(&1));
    set.add(1);
    assert!(!set.remove(&2));
    assert_eq!(set.len(), 1);
}

#[test]
fn test_hash_set_add_remove_cycles() {
    let mut set = HashSet::new();
    for i in 0..10 {
        set.add(i);
        set.add(i);
        assert_eq!(set.len(), 1);
        assert!(set.remove(&i));
        assert!(set.is_empty());
        assert!(!set.remove(&i));
    }
}

// -- BinarySearchTree --

#[test]
fn test_bst_search_delete_nonexistent() {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    assert!(!tree.search(&1));
    assert_eq!(tree.delete(&1), Err(Error::NotFound));
    tree.insert(5);
    assert!(!tree.search(&4));
    assert_eq!(tree.delete(&4), Err(Error::NotFound));
    assert_eq!(tree.len(), 1);
}

#[test]
fn test_bst_insert_duplicates() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(5);
    tree.insert(5);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.in_order(), vec![5]);
}

#[test]
fn test_bst_insert_delete_cycles() {
    let mut tree = BinarySearchTree::new();
    for i in 0..10 {
        tree.insert(i);
        tree.insert(i);
        assert!(tree.search(&i));
        tree.delete(&i).unwrap();
        assert!(!tree.search(&i));
        assert!(tree.is_empty());
    }
}

#[test]
fn test_bst_in_order_after_deletes() {
    let mut tree = BinarySearchTree::new();
    for v in [5, 3, 7, 1, 4, 6, 9] {
        tree.insert(v);
    }
    tree.delete(&3).unwrap(); // node with two children
    tree.delete(&9).unwrap(); // leaf
    tree.delete(&5).unwrap(); // root with two children
    assert_eq!(tree.in_order(), vec![1, 4, 6, 7]);
    assert_eq!(tree.len(), 4);
}

// -- AvlTree --

#[test]
fn test_avl_search_delete_nonexistent() {
    let mut tree: AvlTree<i32> = AvlTree::new();
    assert!(!tree.search(&1));
    assert_eq!(tree.delete(&1), Err(Error::NotFound));
    tree.insert(10);
    assert_eq!(tree.delete(&5), Err(Error::NotFound));
    assert_eq!(tree.len(), 1);
}

#[test]
fn test_avl_insert_duplicates() {
    let mut tree = AvlTree::new();
    for _ in 0..5 {
        tree.insert(3);
    }
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.in_order(), vec![3]);
}

#[test]
fn test_avl_insert_delete_cycles() {
    let mut tree = AvlTree::new();
    for _ in 0..3 {
        for i in 0..50 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 50);
        assert!(tree.height() <= 8);
        for i in (0..50).step_by(2) {
            tree.delete(&i).unwrap();
        }
        assert_eq!(tree.len(), 25);
        assert!(tree.height() <= 6);
        for i in (1..50).step_by(2) {
            tree.delete(&i).unwrap();
        }
        assert!(tree.is_empty());
    }
}

#[test]
fn test_avl_in_order_after_deletes() {
    let mut tree = AvlTree::new();
    for v in [5, 3, 7, 1, 4, 6, 9, 8, 2] {
        tree.insert(v);
    }
    tree.delete(&5).unwrap();
    tree.delete(&1).unwrap();
    tree.delete(&9).unwrap();
    assert_eq!(tree.in_order(), vec![2, 3, 4, 6, 7, 8]);
    // Tree remains balanced.
    assert!(tree.height() <= 4);
}

// -- MinHeap / MaxHeap --

#[test]
fn test_min_heap_pop_empty() {
    let mut heap: MinHeap<i32> = MinHeap::new();
    assert_eq!(heap.pop(), Err(Error::Empty));
    assert_eq!(heap.peek(), Err(Error::Empty));
    heap.push(1);
    assert_eq!(heap.pop(), Ok(1));
    assert_eq!(heap.pop(), Err(Error::Empty));
}

#[test]
fn test_max_heap_pop_empty() {
    let mut heap: MaxHeap<i32> = MaxHeap::new();
    assert_eq!(heap.pop(), Err(Error::Empty));
    assert_eq!(heap.peek(), Err(Error::Empty));
    heap.push(1);
    assert_eq!(heap.pop(), Ok(1));
    assert_eq!(heap.pop(), Err(Error::Empty));
}

#[test]
fn test_min_heap_repeated_push_pop_order() {
    let mut heap = MinHeap::new();
    let mut expected = Vec::new();
    for round in 0..3 {
        for v in [5, 1, 8, 3, 9, 2] {
            heap.push(v + round);
            expected.push(v + round);
        }
        expected.sort();
        for &e in &expected {
            assert_eq!(heap.pop(), Ok(e));
        }
        expected.clear();
        assert!(heap.is_empty());
    }
}

#[test]
fn test_max_heap_repeated_push_pop_order() {
    let mut heap = MaxHeap::new();
    for _ in 0..3 {
        for v in [5, 1, 8, 3, 9, 2] {
            heap.push(v);
        }
        for e in [9, 8, 5, 3, 2, 1] {
            assert_eq!(heap.pop(), Ok(e));
        }
        assert!(heap.is_empty());
    }
}

#[test]
fn test_heaps_duplicate_values() {
    let mut min_heap = MinHeap::new();
    for v in [2, 2, 1, 1, 3] {
        min_heap.push(v);
    }
    assert_eq!(min_heap.len(), 5);
    assert_eq!(min_heap.pop(), Ok(1));
    assert_eq!(min_heap.pop(), Ok(1));
    assert_eq!(min_heap.pop(), Ok(2));
    assert_eq!(min_heap.pop(), Ok(2));
    assert_eq!(min_heap.pop(), Ok(3));

    let mut max_heap = MaxHeap::new();
    for v in [2, 2, 1, 3, 3] {
        max_heap.push(v);
    }
    assert_eq!(max_heap.pop(), Ok(3));
    assert_eq!(max_heap.pop(), Ok(3));
    assert_eq!(max_heap.pop(), Ok(2));
    assert_eq!(max_heap.pop(), Ok(2));
    assert_eq!(max_heap.pop(), Ok(1));
}

// -- Trie --

#[test]
fn test_trie_search_starts_with_nonexistent() {
    let trie = Trie::new();
    assert!(!trie.search("cat"));
    assert!(!trie.starts_with("ca"));
    assert_eq!(trie.words("ca"), Vec::<String>::new());
    assert!(trie.is_empty());
}

#[test]
fn test_trie_delete_nonexistent() {
    let mut trie = Trie::new();
    assert_eq!(trie.delete("cat"), Err(Error::NotFound));
    trie.insert("cat");
    // Deleting a prefix that is not a word fails.
    assert_eq!(trie.delete("ca"), Err(Error::NotFound));
    assert_eq!(trie.len(), 1);
}

#[test]
fn test_trie_repeated_insert_delete() {
    let mut trie = Trie::new();
    for _ in 0..3 {
        trie.insert("hello");
        trie.insert("help");
        assert!(trie.search("hello"));
        trie.delete("hello").unwrap();
        assert!(!trie.search("hello"));
        assert!(trie.search("help"));
        trie.delete("help").unwrap();
        assert!(trie.is_empty());
    }
    // Duplicate inserts count once.
    trie.insert("dup");
    trie.insert("dup");
    assert_eq!(trie.len(), 1);
}

#[test]
fn test_trie_words_prefix() {
    let mut trie = Trie::new();
    trie.insert("bat");
    trie.insert("batch");
    trie.insert("bathe");
    trie.insert("cat");
    let mut words = trie.words("bat");
    words.sort();
    assert_eq!(words, vec!["bat", "batch", "bathe"]);
    assert_eq!(trie.words("xyz"), Vec::<String>::new());
    // Empty prefix returns all words.
    assert_eq!(trie.words("").len(), 4);
    // After deleting, shared prefixes keep other words reachable.
    trie.delete("batch").unwrap();
    let mut words = trie.words("bat");
    words.sort();
    assert_eq!(words, vec!["bat", "bathe"]);
    assert!(trie.starts_with("bat"));
    assert!(!trie.starts_with("batc"));
}

// -- AdjacencyListGraph --

#[test]
fn test_adjacency_list_graph_nonexistent_vertex() {
    let mut g: AdjacencyListGraph<&str> = AdjacencyListGraph::new(false);
    assert_eq!(g.neighbors(&"x"), Vec::<&str>::new());
    assert_eq!(g.remove_vertex(&"x"), Err(Error::NotFound));
    assert!(g.bfs(&"x").is_empty());
    assert!(g.dfs(&"x").is_empty());
    g.add_vertex("a");
    assert_eq!(g.neighbors(&"a"), Vec::<&str>::new());
    assert_eq!(g.remove_vertex(&"b"), Err(Error::NotFound));
}

#[test]
fn test_adjacency_list_graph_isolated_vertex() {
    let mut g = AdjacencyListGraph::new(false);
    g.add_edge("a", "b");
    g.add_vertex("iso");
    assert_eq!(g.bfs(&"iso"), vec!["iso"]);
    assert_eq!(g.dfs(&"iso"), vec!["iso"]);
    g.remove_vertex(&"iso").unwrap();
    assert_eq!(g.remove_vertex(&"iso"), Err(Error::NotFound));
}

#[test]
fn test_adjacency_list_graph_add_remove_edge_cycles() {
    let mut g = AdjacencyListGraph::new(false);
    for _ in 0..3 {
        g.add_edge("a", "b");
        g.add_edge("a", "b"); // duplicate edge is a no-op
        assert!(g.has_edge(&"a", &"b"));
        assert!(g.has_edge(&"b", &"a"));
        assert_eq!(g.neighbors(&"a"), vec!["b"]);
        g.remove_edge(&"a", &"b");
        assert!(!g.has_edge(&"a", &"b"));
        assert!(!g.has_edge(&"b", &"a"));
    }
    // Removing a vertex also removes edges to it.
    g.add_edge("x", "y");
    g.add_edge("z", "y");
    g.remove_vertex(&"y").unwrap();
    assert!(!g.has_edge(&"x", &"y"));
    assert!(!g.has_edge(&"z", &"y"));
    assert_eq!(g.vertex_count(), 4);
}

// -- AdjacencyMatrixGraph --

#[test]
fn test_adjacency_matrix_graph_nonexistent_vertex() {
    let mut g: AdjacencyMatrixGraph<i32> = AdjacencyMatrixGraph::new(false);
    assert_eq!(g.neighbors(&9), Vec::<i32>::new());
    assert_eq!(g.remove_vertex(&9), Err(Error::NotFound));
    assert_eq!(g.weight(&1, &2), Err(Error::NotFound));
    assert!(g.bfs(&9).is_empty());
    assert!(g.dfs(&9).is_empty());
    g.add_vertex(1);
    assert_eq!(g.weight(&1, &9), Err(Error::NotFound));
}

#[test]
fn test_adjacency_matrix_graph_isolated_vertex() {
    let mut g = AdjacencyMatrixGraph::new(false);
    g.add_edge(1, 2, 5);
    g.add_vertex(99);
    assert_eq!(g.bfs(&99), vec![99]);
    assert_eq!(g.dfs(&99), vec![99]);
    g.remove_vertex(&99).unwrap();
    assert_eq!(g.remove_vertex(&99), Err(Error::NotFound));
    // Remaining graph is intact after removal.
    assert_eq!(g.weight(&1, &2), Ok(5));
    assert_eq!(g.neighbors(&1), vec![2]);
}

#[test]
fn test_adjacency_matrix_graph_add_remove_edge_cycles() {
    let mut g = AdjacencyMatrixGraph::new(false);
    for _ in 0..3 {
        g.add_edge(1, 2, 7);
        assert!(g.has_edge(&1, &2));
        assert!(g.has_edge(&2, &1));
        assert_eq!(g.weight(&1, &2), Ok(7));
        g.remove_edge(&1, &2);
        assert!(!g.has_edge(&1, &2));
        assert!(!g.has_edge(&2, &1));
        assert_eq!(g.weight(&1, &2), Ok(0));
    }
    // Removing a vertex reindexes the matrix without corrupting other edges.
    g.add_edge(1, 2, 1);
    g.add_edge(2, 3, 2);
    g.add_edge(3, 4, 3);
    g.remove_vertex(&2).unwrap();
    assert!(!g.has_edge(&1, &2));
    assert!(g.has_edge(&3, &4));
    assert_eq!(g.weight(&3, &4), Ok(3));
}

// -- LruCache --

#[test]
fn test_lru_cache_get_nonexistent() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2).unwrap();
    assert_eq!(cache.get(&"missing"), Err(Error::NotFound));
    assert!(!cache.has(&"missing"));
    cache.set("a", 1);
    assert_eq!(cache.get(&"b"), Err(Error::NotFound));
}

#[test]
fn test_lru_cache_zero_capacity_rejected() {
    let result = LruCache::<&str, i32>::new(0);
    assert!(matches!(result, Err(Error::InvalidArgument)));
}

#[test]
fn test_lru_cache_eviction_order() {
    let mut cache = LruCache::new(2).unwrap();
    for _ in 0..3 {
        cache.set("a", 1);
        cache.set("b", 2);
        cache.get(&"a").unwrap(); // refresh "a"; "b" is now oldest
        cache.set("c", 3); // evicts "b"
        assert!(cache.has(&"a"));
        assert!(!cache.has(&"b"));
        assert!(cache.has(&"c"));
        cache.get(&"a").unwrap();
        cache.set("d", 4); // evicts "c"
        assert!(cache.has(&"a"));
        assert!(!cache.has(&"c"));
        assert!(cache.has(&"d"));
        assert_eq!(cache.len(), 2);
    }
    // Overwriting an existing key does not grow the cache or change order.
    cache.set("a", 100);
    assert_eq!(cache.get(&"a"), Ok(&100));
    assert_eq!(cache.len(), 2);
}

// -- BloomFilter --

#[test]
fn test_bloom_filter_invalid_arguments() {
    assert!(matches!(BloomFilter::new(0, 0.05), Err(Error::InvalidArgument)));
    assert!(matches!(
        BloomFilter::new(100, 0.0),
        Err(Error::InvalidArgument)
    ));
    assert!(matches!(
        BloomFilter::new(100, 1.0),
        Err(Error::InvalidArgument)
    ));
    assert!(matches!(
        BloomFilter::new(100, -0.5),
        Err(Error::InvalidArgument)
    ));
    assert!(matches!(
        BloomFilter::new(100, 1.5),
        Err(Error::InvalidArgument)
    ));
}

#[test]
fn test_bloom_filter_no_false_negatives() {
    let mut bf = BloomFilter::new(200, 0.01).unwrap();
    let items: Vec<String> = (0..150).map(|i| format!("string_item_{}", i)).collect();
    for item in &items {
        bf.add(item);
    }
    // Bloom filters never produce false negatives.
    for item in &items {
        assert!(bf.has(item), "false negative for {}", item);
    }
    assert_eq!(bf.count(), 150);
}

#[test]
fn test_bloom_filter_never_added_item() {
    let mut bf = BloomFilter::new(1000, 0.001).unwrap();
    for i in 0..10 {
        bf.add(&format!("present_{}", i));
    }
    // A clearly different item should (with very high probability) be absent.
    // With 1000 slots, 10 items, and a 0.001 target fpp this is safe to assert.
    assert!(!bf.has("definitely_never_added_zzz"));
    // has() on an empty filter is always false.
    let empty = BloomFilter::new(100, 0.05).unwrap();
    assert!(!empty.has("anything"));
}
