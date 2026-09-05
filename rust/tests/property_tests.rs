use data_structures_lib::{
    AdjacencyListGraph, BloomFilter, CircularLinkedList, DoublyLinkedList, HashMap, HashSet,
    SinglyLinkedList,
};
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet, VecDeque};

#[test]
fn hash_map_properties() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut lib_map = HashMap::new();
    let mut ref_map = StdHashMap::new();

    let key_range = -10_000i64..10_000i64;
    let value_range = -1_000_000i64..1_000_000i64;
    let missing_key = i64::MAX;

    for _ in 0..1000 {
        let key = rng.gen_range(key_range.clone());
        let value = rng.gen_range(value_range.clone());

        lib_map.set(key, value);
        ref_map.insert(key, value);

        assert_eq!(lib_map.len(), ref_map.len());
        assert_eq!(lib_map.get(&key), ref_map.get(&key));
        assert_eq!(lib_map.get(&missing_key), None);
    }

    let distinct_keys: Vec<i64> = ref_map.keys().copied().collect();
    for key in &distinct_keys {
        if rng.gen_bool(0.2) {
            let lib_deleted = lib_map.delete(key);
            let ref_deleted = ref_map.remove(key);
            assert_eq!(lib_deleted, ref_deleted);
            assert_eq!(lib_map.len(), ref_map.len());
        }
    }

    assert_eq!(lib_map.len(), ref_map.len());
    for (key, value) in &ref_map {
        assert_eq!(lib_map.get(key), Some(value));
    }
    assert_eq!(lib_map.get(&missing_key), None);
}

#[test]
fn hash_set_properties() {
    let mut rng = StdRng::seed_from_u64(43);
    let mut lib_set = HashSet::new();
    let mut ref_set = StdHashSet::new();

    let value_range = -10_000i64..10_000i64;
    let missing_value = i64::MAX;

    for _ in 0..1000 {
        let value = rng.gen_range(value_range.clone());

        lib_set.add(value);
        ref_set.insert(value);

        assert_eq!(lib_set.len(), ref_set.len());
        assert_eq!(lib_set.has(&value), ref_set.contains(&value));
    }

    let distinct_values: Vec<i64> = ref_set.iter().copied().collect();
    for value in &distinct_values {
        if rng.gen_bool(0.2) {
            let lib_removed = lib_set.remove(value);
            let ref_removed = ref_set.remove(value);
            assert_eq!(lib_removed, ref_removed);
            assert_eq!(lib_set.len(), ref_set.len());
        }
    }

    assert_eq!(lib_set.len(), ref_set.len());
    let lib_values: StdHashSet<i64> = lib_set.to_vec().into_iter().cloned().collect();
    assert_eq!(lib_values, ref_set);
    assert!(!lib_set.has(&missing_value));
}

#[test]
fn bloom_filter_properties() {
    let mut rng = StdRng::seed_from_u64(44);
    let mut bf = BloomFilter::new(1000, 0.01).unwrap();

    let mut inserted = StdHashSet::new();
    while inserted.len() < 500 {
        let len = rng.gen_range(8usize..=16);
        let s: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        if inserted.insert(s.clone()) {
            bf.add(&s);
        }
    }

    for s in &inserted {
        assert!(bf.has(s), "inserted item should be present: {}", s);
    }

    let mut fresh = StdHashSet::new();
    while fresh.len() < 5000 {
        let len = rng.gen_range(8usize..=16);
        let s: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        if !inserted.contains(&s) {
            fresh.insert(s);
        }
    }

    let false_positives = fresh.iter().filter(|s| bf.has(s)).count();
    let observed_fpp = false_positives as f64 / fresh.len() as f64;
    assert!(
        observed_fpp < 0.05,
        "observed false-positive rate too high: {}",
        observed_fpp
    );
    assert!(bf.expected_fpp() < 0.05, "expected fpp too high: {}", bf.expected_fpp());
}

fn reference_reachable(adj: &[Vec<i32>], start: i32) -> StdHashSet<i32> {
    let mut visited = StdHashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        if visited.insert(u) {
            for &v in &adj[u as usize] {
                if !visited.contains(&v) {
                    queue.push_back(v);
                }
            }
        }
    }

    visited
}

#[test]
fn graph_bfs_dfs_properties() {
    let mut rng = StdRng::seed_from_u64(45);

    for _ in 0..10 {
        let n = rng.gen_range(5i32..=20);
        let p = rng.gen_range(0.1f64..0.3f64);

        let mut graph = AdjacencyListGraph::new(false);
        let mut adj = vec![Vec::new(); n as usize];

        for v in 0..n {
            graph.add_vertex(v);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                if rng.gen::<f64>() < p {
                    graph.add_edge(i, j);
                    adj[i as usize].push(j);
                    adj[j as usize].push(i);
                }
            }
        }

        let source_count = (n as usize).clamp(1, 8);
        for _ in 0..source_count {
            let source = rng.gen_range(0..n);
            let lib_bfs: StdHashSet<i32> = graph.bfs(&source).into_iter().collect();
            let lib_dfs: StdHashSet<i32> = graph.dfs(&source).into_iter().collect();
            let expected = reference_reachable(&adj, source);

            assert_eq!(
                lib_bfs, expected,
                "BFS reachable set mismatch from source {}",
                source
            );
            assert_eq!(
                lib_dfs, expected,
                "DFS reachable set mismatch from source {}",
                source
            );
        }
    }
}

fn run_linked_list_cycles<L, F>(make_list: F)
where
    L: LinkedListOps,
    F: FnOnce() -> L,
{
    let mut rng = StdRng::seed_from_u64(46);
    let mut list = make_list();
    let mut reference: Vec<i64> = Vec::new();

    for _ in 0..500 {
        let action = if reference.is_empty() {
            0
        } else {
            rng.gen_range(0..3)
        };

        match action {
            0 => {
                let value = rng.gen_range(-1000i64..1000);
                list.append(value);
                reference.push(value);
            }
            1 => {
                let idx = rng.gen_range(0..reference.len());
                let expected = reference[idx];
                let actual = list.pop(Some(idx)).unwrap();
                assert_eq!(actual, expected);
                reference.remove(idx);
            }
            2 => {
                let idx = rng.gen_range(0..reference.len());
                let value = reference[idx];
                list.remove(&value).unwrap();
                let pos = reference.iter().position(|x| *x == value).unwrap();
                reference.remove(pos);
            }
            _ => unreachable!(),
        }

        assert_eq!(list.len(), reference.len());
        assert_eq!(list.to_vec(), reference);
    }
}

trait LinkedListOps {
    fn append(&mut self, value: i64);
    fn remove(&mut self, value: &i64) -> Result<(), data_structures_lib::Error>;
    fn pop(&mut self, index: Option<usize>) -> Result<i64, data_structures_lib::Error>;
    fn len(&self) -> usize;
    fn to_vec(&self) -> Vec<i64>;
}

impl LinkedListOps for SinglyLinkedList<i64> {
    fn append(&mut self, value: i64) {
        SinglyLinkedList::append(self, value);
    }

    fn remove(&mut self, value: &i64) -> Result<(), data_structures_lib::Error> {
        SinglyLinkedList::remove(self, value)
    }

    fn pop(&mut self, index: Option<usize>) -> Result<i64, data_structures_lib::Error> {
        SinglyLinkedList::pop(self, index)
    }

    fn len(&self) -> usize {
        SinglyLinkedList::len(self)
    }

    fn to_vec(&self) -> Vec<i64> {
        SinglyLinkedList::to_vec(self)
    }
}

impl LinkedListOps for DoublyLinkedList<i64> {
    fn append(&mut self, value: i64) {
        DoublyLinkedList::append(self, value);
    }

    fn remove(&mut self, value: &i64) -> Result<(), data_structures_lib::Error> {
        DoublyLinkedList::remove(self, value)
    }

    fn pop(&mut self, index: Option<usize>) -> Result<i64, data_structures_lib::Error> {
        DoublyLinkedList::pop(self, index)
    }

    fn len(&self) -> usize {
        DoublyLinkedList::len(self)
    }

    fn to_vec(&self) -> Vec<i64> {
        DoublyLinkedList::to_vec(self)
    }
}

#[test]
fn singly_linked_list_repeated_cycles() {
    run_linked_list_cycles(SinglyLinkedList::<i64>::new);
}

#[test]
fn doubly_linked_list_repeated_cycles() {
    run_linked_list_cycles(DoublyLinkedList::<i64>::new);
}

#[test]
fn circular_linked_list_repeated_cycles() {
    let mut rng = StdRng::seed_from_u64(47);
    let mut list = CircularLinkedList::new();
    let mut reference: Vec<i64> = Vec::new();

    for _ in 0..300 {
        let action = if reference.is_empty() {
            0
        } else {
            rng.gen_range(0..3)
        };

        match action {
            0 => {
                let value = rng.gen_range(-1000i64..1000);
                list.append(value);
                reference.push(value);
            }
            1 => {
                let value = rng.gen_range(-1000i64..1000);
                list.prepend(value);
                reference.insert(0, value);
            }
            2 => {
                let idx = rng.gen_range(0..reference.len());
                let value = reference[idx];
                list.remove(&value).unwrap();
                let pos = reference.iter().position(|x| *x == value).unwrap();
                reference.remove(pos);
            }
            _ => unreachable!(),
        }

        assert_eq!(list.len(), reference.len());
        assert_eq!(list.to_vec(), reference);
    }
}
