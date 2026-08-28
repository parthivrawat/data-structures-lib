package datastructures

import (
	"container/list"
)

// LRUCache is a least-recently-used cache with a fixed capacity.
type LRUCache[K comparable, V any] struct {
	capacity int
	cache    map[K]*list.Element
	order    *list.List
}

type lruEntry[K comparable, V any] struct {
	key   K
	value V
}

// NewLRUCache creates a new LRUCache.
func NewLRUCache[K comparable, V any](capacity int) (*LRUCache[K, V], error) {
	if capacity <= 0 {
		return nil, ErrInvalidCapacity
	}
	return &LRUCache[K, V]{
		capacity: capacity,
		cache:    make(map[K]*list.Element),
		order:    list.New(),
	}, nil
}

// Len returns the number of entries.
func (c *LRUCache[K, V]) Len() int { return c.order.Len() }

// Has returns true if the key exists.
func (c *LRUCache[K, V]) Has(key K) bool {
	_, ok := c.cache[key]
	return ok
}

// Get returns the value for a key.
func (c *LRUCache[K, V]) Get(key K) (V, error) {
	var zero V
	elem, ok := c.cache[key]
	if !ok {
		return zero, ErrKeyNotFound
	}
	c.order.MoveToFront(elem)
	return elem.Value.(*lruEntry[K, V]).value, nil
}

// Set sets a key-value pair.
func (c *LRUCache[K, V]) Set(key K, value V) {
	if elem, ok := c.cache[key]; ok {
		c.order.MoveToFront(elem)
		elem.Value.(*lruEntry[K, V]).value = value
		return
	}
	if c.order.Len() >= c.capacity {
		oldest := c.order.Back()
		if oldest != nil {
			entry := oldest.Value.(*lruEntry[K, V])
			delete(c.cache, entry.key)
			c.order.Remove(oldest)
		}
	}
	entry := &lruEntry[K, V]{key: key, value: value}
	elem := c.order.PushFront(entry)
	c.cache[key] = elem
}
