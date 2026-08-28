package datastructures

import (
	"hash/fnv"
	"math"
)

// BloomFilter is a Bloom filter for membership queries.
type BloomFilter struct {
	size       int
	hashCount  int
	bits       []byte
	itemsAdded int
}

// NewBloomFilter creates a new BloomFilter.
func NewBloomFilter(expectedItems int, falsePositiveRate float64) (*BloomFilter, error) {
	if expectedItems <= 0 {
		return nil, ErrInvalidCapacity
	}
	if falsePositiveRate <= 0 || falsePositiveRate >= 1 {
		return nil, ErrNotFound
	}
	m := int(math.Ceil(-(float64(expectedItems) * math.Log(falsePositiveRate)) / (math.Pow(math.Log(2), 2))))
	k := max(1, int(math.Round((float64(m)/float64(expectedItems))*math.Log(2))))
	return &BloomFilter{
		size:      m,
		hashCount: k,
		bits:      make([]byte, (m+7)/8),
	}, nil
}

// Add adds an item to the filter.
func (b *BloomFilter) Add(item string) {
	for i := 0; i < b.hashCount; i++ {
		position := b.hash(item, i)
		b.bits[position/8] |= 1 << (position % 8)
	}
	b.itemsAdded++
}

// Has returns true if the item might be in the filter.
func (b *BloomFilter) Has(item string) bool {
	for i := 0; i < b.hashCount; i++ {
		position := b.hash(item, i)
		if b.bits[position/8]&(1<<(position%8)) == 0 {
			return false
		}
	}
	return true
}

// Count returns the number of items added.
func (b *BloomFilter) Count() int { return b.itemsAdded }

// ExpectedFpp returns the estimated false-positive probability.
func (b *BloomFilter) ExpectedFpp() float64 {
	return math.Pow(1-math.Exp(-float64(b.hashCount*b.itemsAdded)/float64(b.size)), float64(b.hashCount))
}

func (b *BloomFilter) hash(item string, seed int) int {
	h := fnv.New64a()
	h.Write([]byte(item))
	h.Write([]byte{byte(seed)})
	return int(h.Sum64() % uint64(b.size))
}
