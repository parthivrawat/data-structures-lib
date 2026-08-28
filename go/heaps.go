package datastructures

// Heap is a generic binary heap.
type Heap[T any] struct {
	data []T
	less func(a, b T) bool
}

// NewHeap creates a new Heap with the given comparison function.
func NewHeap[T any](less func(a, b T) bool) *Heap[T] {
	return &Heap[T]{less: less}
}

// Len returns the number of elements.
func (h *Heap[T]) Len() int { return len(h.data) }

// IsEmpty returns true if the heap is empty.
func (h *Heap[T]) IsEmpty() bool { return len(h.data) == 0 }

// Push adds a value.
func (h *Heap[T]) Push(value T) {
	h.data = append(h.data, value)
	h.siftUp(len(h.data) - 1)
}

// Pop removes and returns the top value.
func (h *Heap[T]) Pop() (T, error) {
	var zero T
	if h.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	if len(h.data) == 1 {
		return h.data[0], nil
	}
	root := h.data[0]
	h.data[0] = h.data[len(h.data)-1]
	h.data = h.data[:len(h.data)-1]
	h.siftDown(0)
	return root, nil
}

// Peek returns the top value without removing it.
func (h *Heap[T]) Peek() (T, error) {
	var zero T
	if h.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	return h.data[0], nil
}

// ToSlice returns a slice of the heap's data.
func (h *Heap[T]) ToSlice() []T {
	result := make([]T, len(h.data))
	copy(result, h.data)
	return result
}

func (h *Heap[T]) siftUp(index int) {
	current := index
	for current > 0 {
		parent := (current - 1) / 2
		if h.less(h.data[current], h.data[parent]) {
			h.data[current], h.data[parent] = h.data[parent], h.data[current]
			current = parent
		} else {
			break
		}
	}
}

func (h *Heap[T]) siftDown(index int) {
	length := len(h.data)
	current := index
	for {
		target := current
		left := 2*current + 1
		right := 2*current + 2
		if left < length && h.less(h.data[left], h.data[target]) {
			target = left
		}
		if right < length && h.less(h.data[right], h.data[target]) {
			target = right
		}
		if target == current {
			break
		}
		h.data[current], h.data[target] = h.data[target], h.data[current]
		current = target
	}
}

// MinHeap is a min-heap for ordered types.
type MinHeap[T Ordered] struct {
	*Heap[T]
}

// NewMinHeap creates a new MinHeap.
func NewMinHeap[T Ordered]() *MinHeap[T] {
	return &MinHeap[T]{Heap: NewHeap[T](func(a, b T) bool { return a < b })}
}

// MaxHeap is a max-heap for ordered types.
type MaxHeap[T Ordered] struct {
	*Heap[T]
}

// NewMaxHeap creates a new MaxHeap.
func NewMaxHeap[T Ordered]() *MaxHeap[T] {
	return &MaxHeap[T]{Heap: NewHeap[T](func(a, b T) bool { return a > b })}
}
