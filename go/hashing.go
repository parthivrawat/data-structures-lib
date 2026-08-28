package datastructures

// HashMap is a key-value map with O(1) average access time.
type HashMap[K comparable, V any] struct {
	data map[K]V
}

// NewHashMap creates a new HashMap.
func NewHashMap[K comparable, V any]() *HashMap[K, V] {
	return &HashMap[K, V]{data: make(map[K]V)}
}

// Set sets a key-value pair.
func (m *HashMap[K, V]) Set(key K, value V) {
	m.data[key] = value
}

// Get returns the value for a key and whether it exists.
func (m *HashMap[K, V]) Get(key K) (V, bool) {
	value, ok := m.data[key]
	return value, ok
}

// Has returns true if the key exists.
func (m *HashMap[K, V]) Has(key K) bool {
	_, ok := m.data[key]
	return ok
}

// Delete removes a key.
func (m *HashMap[K, V]) Delete(key K) {
	delete(m.data, key)
}

// Keys returns all keys.
func (m *HashMap[K, V]) Keys() []K {
	keys := make([]K, 0, len(m.data))
	for k := range m.data {
		keys = append(keys, k)
	}
	return keys
}

// Values returns all values.
func (m *HashMap[K, V]) Values() []V {
	values := make([]V, 0, len(m.data))
	for _, v := range m.data {
		values = append(values, v)
	}
	return values
}

// Entries returns all key-value pairs.
func (m *HashMap[K, V]) Entries() []struct {
	Key   K
	Value V
} {
	entries := make([]struct {
		Key   K
		Value V
	}, 0, len(m.data))
	for k, v := range m.data {
		entries = append(entries, struct {
			Key   K
			Value V
		}{Key: k, Value: v})
	}
	return entries
}

// Len returns the number of pairs.
func (m *HashMap[K, V]) Len() int {
	return len(m.data)
}

// IsEmpty returns true if the map is empty.
func (m *HashMap[K, V]) IsEmpty() bool {
	return len(m.data) == 0
}

// HashSet is a set of unique values.
type HashSet[T comparable] struct {
	data map[T]struct{}
}

// NewHashSet creates a new HashSet.
func NewHashSet[T comparable]() *HashSet[T] {
	return &HashSet[T]{data: make(map[T]struct{})}
}

// Add adds a value.
func (s *HashSet[T]) Add(value T) {
	s.data[value] = struct{}{}
}

// Remove removes a value, returning an error if not present.
func (s *HashSet[T]) Remove(value T) error {
	if !s.Has(value) {
		return ErrNotFound
	}
	delete(s.data, value)
	return nil
}

// Discard removes a value if present.
func (s *HashSet[T]) Discard(value T) {
	delete(s.data, value)
}

// Has returns true if the value is in the set.
func (s *HashSet[T]) Has(value T) bool {
	_, ok := s.data[value]
	return ok
}

// Len returns the number of values.
func (s *HashSet[T]) Len() int {
	return len(s.data)
}

// IsEmpty returns true if the set is empty.
func (s *HashSet[T]) IsEmpty() bool {
	return len(s.data) == 0
}

// ToSlice returns a slice of all values.
func (s *HashSet[T]) ToSlice() []T {
	result := make([]T, 0, len(s.data))
	for k := range s.data {
		result = append(result, k)
	}
	return result
}
