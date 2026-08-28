package datastructures

import (
	"reflect"
)

// DynamicArray is a dynamic array with automatic resizing.
type DynamicArray[T any] struct {
	data     []T
	size     int
	capacity int
}

// NewDynamicArray creates a new DynamicArray.
func NewDynamicArray[T any](initialCapacity ...int) *DynamicArray[T] {
	c := 10
	if len(initialCapacity) > 0 && initialCapacity[0] > 0 {
		c = initialCapacity[0]
	}
	return &DynamicArray[T]{
		data:     make([]T, c),
		size:     0,
		capacity: c,
	}
}

// Len returns the number of elements.
func (a *DynamicArray[T]) Len() int { return a.size }

// Cap returns the current capacity.
func (a *DynamicArray[T]) Cap() int { return a.capacity }

// IsEmpty returns true if the array is empty.
func (a *DynamicArray[T]) IsEmpty() bool { return a.size == 0 }

// Append adds a value to the end.
func (a *DynamicArray[T]) Append(value T) {
	if a.size == a.capacity {
		a.resize(2 * a.capacity)
	}
	a.data[a.size] = value
	a.size++
}

// Insert inserts a value at the given index.
func (a *DynamicArray[T]) Insert(index int, value T) error {
	if index < 0 || index > a.size {
		return ErrIndexOutOfRange
	}
	if a.size == a.capacity {
		a.resize(2 * a.capacity)
	}
	for i := a.size; i > index; i-- {
		a.data[i] = a.data[i-1]
	}
	a.data[index] = value
	a.size++
	return nil
}

// Remove removes the first occurrence of value.
func (a *DynamicArray[T]) Remove(value T) error {
	for i := 0; i < a.size; i++ {
		if reflect.DeepEqual(a.data[i], value) {
			a.Pop(i)
			return nil
		}
	}
	return ErrNotFound
}

// Pop removes and returns the item at index. Defaults to the last item.
func (a *DynamicArray[T]) Pop(index ...int) (T, error) {
	var zero T
	if a.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	idx := -1
	if len(index) > 0 {
		idx = index[0]
	}
	if idx < 0 {
		idx += a.size
	}
	if idx < 0 || idx >= a.size {
		return zero, ErrIndexOutOfRange
	}
	value := a.data[idx]
	for i := idx; i < a.size-1; i++ {
		a.data[i] = a.data[i+1]
	}
	a.size--
	if a.size > 0 && a.size == a.capacity/4 && a.capacity > 10 {
		a.resize(a.capacity / 2)
	}
	return value, nil
}

// Get returns the item at index.
func (a *DynamicArray[T]) Get(index int) (T, error) {
	var zero T
	if index < 0 || index >= a.size {
		return zero, ErrIndexOutOfRange
	}
	return a.data[index], nil
}

// Set sets the item at index.
func (a *DynamicArray[T]) Set(index int, value T) error {
	if index < 0 || index >= a.size {
		return ErrIndexOutOfRange
	}
	a.data[index] = value
	return nil
}

// ToSlice returns a new slice with the current elements.
func (a *DynamicArray[T]) ToSlice() []T {
	result := make([]T, a.size)
	copy(result, a.data[:a.size])
	return result
}

func (a *DynamicArray[T]) resize(newCapacity int) {
	newData := make([]T, newCapacity)
	copy(newData, a.data[:a.size])
	a.data = newData
	a.capacity = newCapacity
}

// sNode is a singly linked list node.
type sNode[T any] struct {
	value T
	next  *sNode[T]
}

// SinglyLinkedList is a singly linked list.
type SinglyLinkedList[T any] struct {
	head *sNode[T]
	size int
}

// NewSinglyLinkedList creates a new SinglyLinkedList.
func NewSinglyLinkedList[T any]() *SinglyLinkedList[T] {
	return &SinglyLinkedList[T]{}
}

// Len returns the number of elements.
func (l *SinglyLinkedList[T]) Len() int { return l.size }

// IsEmpty returns true if the list is empty.
func (l *SinglyLinkedList[T]) IsEmpty() bool { return l.size == 0 }

// Append adds a value to the end.
func (l *SinglyLinkedList[T]) Append(value T) {
	node := &sNode[T]{value: value}
	if l.head == nil {
		l.head = node
	} else {
		current := l.head
		for current.next != nil {
			current = current.next
		}
		current.next = node
	}
	l.size++
}

// Prepend adds a value to the front.
func (l *SinglyLinkedList[T]) Prepend(value T) {
	node := &sNode[T]{value: value, next: l.head}
	l.head = node
	l.size++
}

// Insert inserts a value at index.
func (l *SinglyLinkedList[T]) Insert(index int, value T) error {
	if index < 0 || index > l.size {
		return ErrIndexOutOfRange
	}
	if index == 0 {
		l.Prepend(value)
		return nil
	}
	prev := l.nodeAt(index - 1)
	node := &sNode[T]{value: value, next: prev.next}
	prev.next = node
	l.size++
	return nil
}

// Remove removes the first occurrence of value.
func (l *SinglyLinkedList[T]) Remove(value T) error {
	if l.head == nil {
		return ErrNotFound
	}
	if reflect.DeepEqual(l.head.value, value) {
		l.head = l.head.next
		l.size--
		return nil
	}
	current := l.head
	for current.next != nil {
		if reflect.DeepEqual(current.next.value, value) {
			current.next = current.next.next
			l.size--
			return nil
		}
		current = current.next
	}
	return ErrNotFound
}

// Pop removes and returns the item at index.
func (l *SinglyLinkedList[T]) Pop(index int) (T, error) {
	var zero T
	if l.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	if index < 0 {
		index += l.size
	}
	if index < 0 || index >= l.size {
		return zero, ErrIndexOutOfRange
	}
	if index == 0 {
		value := l.head.value
		l.head = l.head.next
		l.size--
		return value, nil
	}
	prev := l.nodeAt(index - 1)
	node := prev.next
	value := node.value
	prev.next = node.next
	l.size--
	return value, nil
}

// Get returns the item at index.
func (l *SinglyLinkedList[T]) Get(index int) (T, error) {
	var zero T
	if index < 0 || index >= l.size {
		return zero, ErrIndexOutOfRange
	}
	return l.nodeAt(index).value, nil
}

// Set sets the item at index.
func (l *SinglyLinkedList[T]) Set(index int, value T) error {
	if index < 0 || index >= l.size {
		return ErrIndexOutOfRange
	}
	l.nodeAt(index).value = value
	return nil
}

// Find returns the index of the first occurrence of value.
func (l *SinglyLinkedList[T]) Find(value T) (int, error) {
	current := l.head
	i := 0
	for current != nil {
		if reflect.DeepEqual(current.value, value) {
			return i, nil
		}
		current = current.next
		i++
	}
	return -1, ErrNotFound
}

// ToSlice returns a slice of all values.
func (l *SinglyLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, l.size)
	current := l.head
	for current != nil {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (l *SinglyLinkedList[T]) nodeAt(index int) *sNode[T] {
	current := l.head
	for i := 0; i < index; i++ {
		current = current.next
	}
	return current
}

// dNode is a doubly linked list node.
type dNode[T any] struct {
	value T
	prev  *dNode[T]
	next  *dNode[T]
}

// DoublyLinkedList is a doubly linked list.
type DoublyLinkedList[T any] struct {
	head *dNode[T]
	tail *dNode[T]
	size int
}

// NewDoublyLinkedList creates a new DoublyLinkedList.
func NewDoublyLinkedList[T any]() *DoublyLinkedList[T] {
	return &DoublyLinkedList[T]{}
}

// Len returns the number of elements.
func (l *DoublyLinkedList[T]) Len() int { return l.size }

// IsEmpty returns true if the list is empty.
func (l *DoublyLinkedList[T]) IsEmpty() bool { return l.size == 0 }

// Append adds a value to the end.
func (l *DoublyLinkedList[T]) Append(value T) {
	node := &dNode[T]{value: value}
	if l.tail == nil {
		l.head = node
		l.tail = node
	} else {
		node.prev = l.tail
		l.tail.next = node
		l.tail = node
	}
	l.size++
}

// Prepend adds a value to the front.
func (l *DoublyLinkedList[T]) Prepend(value T) {
	node := &dNode[T]{value: value}
	if l.head == nil {
		l.head = node
		l.tail = node
	} else {
		node.next = l.head
		l.head.prev = node
		l.head = node
	}
	l.size++
}

// Insert inserts a value at index.
func (l *DoublyLinkedList[T]) Insert(index int, value T) error {
	if index < 0 || index > l.size {
		return ErrIndexOutOfRange
	}
	if index == 0 {
		l.Prepend(value)
		return nil
	}
	if index == l.size {
		l.Append(value)
		return nil
	}
	next := l.nodeAt(index)
	prev := next.prev
	node := &dNode[T]{value: value, prev: prev, next: next}
	prev.next = node
	next.prev = node
	l.size++
	return nil
}

// Remove removes the first occurrence of value.
func (l *DoublyLinkedList[T]) Remove(value T) error {
	current := l.head
	for current != nil {
		if reflect.DeepEqual(current.value, value) {
			l.removeNode(current)
			return nil
		}
		current = current.next
	}
	return ErrNotFound
}

// Pop removes and returns the item at index.
func (l *DoublyLinkedList[T]) Pop(index int) (T, error) {
	var zero T
	if l.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	if index < 0 {
		index += l.size
	}
	if index < 0 || index >= l.size {
		return zero, ErrIndexOutOfRange
	}
	node := l.nodeAt(index)
	l.removeNode(node)
	return node.value, nil
}

// Get returns the item at index.
func (l *DoublyLinkedList[T]) Get(index int) (T, error) {
	var zero T
	if index < 0 || index >= l.size {
		return zero, ErrIndexOutOfRange
	}
	return l.nodeAt(index).value, nil
}

// Set sets the item at index.
func (l *DoublyLinkedList[T]) Set(index int, value T) error {
	if index < 0 || index >= l.size {
		return ErrIndexOutOfRange
	}
	l.nodeAt(index).value = value
	return nil
}

// ToSlice returns a slice of all values.
func (l *DoublyLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, l.size)
	current := l.head
	for current != nil {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (l *DoublyLinkedList[T]) nodeAt(index int) *dNode[T] {
	if index < l.size/2 {
		current := l.head
		for i := 0; i < index; i++ {
			current = current.next
		}
		return current
	}
	current := l.tail
	for i := l.size - 1; i > index; i-- {
		current = current.prev
	}
	return current
}

func (l *DoublyLinkedList[T]) removeNode(node *dNode[T]) {
	if node.prev != nil {
		node.prev.next = node.next
	} else {
		l.head = node.next
	}
	if node.next != nil {
		node.next.prev = node.prev
	} else {
		l.tail = node.prev
	}
	l.size--
}

// cNode is a circular linked list node.
type cNode[T any] struct {
	value T
	next  *cNode[T]
}

// CircularLinkedList is a circular linked list.
type CircularLinkedList[T any] struct {
	tail *cNode[T]
	size int
}

// NewCircularLinkedList creates a new CircularLinkedList.
func NewCircularLinkedList[T any]() *CircularLinkedList[T] {
	return &CircularLinkedList[T]{}
}

// Len returns the number of elements.
func (l *CircularLinkedList[T]) Len() int { return l.size }

// IsEmpty returns true if the list is empty.
func (l *CircularLinkedList[T]) IsEmpty() bool { return l.size == 0 }

// Append adds a value.
func (l *CircularLinkedList[T]) Append(value T) {
	node := &cNode[T]{value: value}
	if l.tail == nil {
		node.next = node
		l.tail = node
	} else {
		node.next = l.tail.next
		l.tail.next = node
		l.tail = node
	}
	l.size++
}

// Prepend adds a value to the front.
func (l *CircularLinkedList[T]) Prepend(value T) {
	node := &cNode[T]{value: value}
	if l.tail == nil {
		node.next = node
		l.tail = node
	} else {
		node.next = l.tail.next
		l.tail.next = node
	}
	l.size++
}

// Remove removes the first occurrence of value.
func (l *CircularLinkedList[T]) Remove(value T) error {
	if l.IsEmpty() {
		return ErrNotFound
	}
	head := l.tail.next
	if reflect.DeepEqual(head.value, value) {
		if head == l.tail {
			l.tail = nil
		} else {
			l.tail.next = head.next
		}
		l.size--
		return nil
	}
	current := head
	for i := 0; i < l.size-1; i++ {
		if reflect.DeepEqual(current.next.value, value) {
			current.next = current.next.next
			if current.next == head {
				l.tail = current
			}
			l.size--
			return nil
		}
		current = current.next
	}
	return ErrNotFound
}

// ToSlice returns a slice of all values.
func (l *CircularLinkedList[T]) ToSlice() []T {
	if l.IsEmpty() {
		return nil
	}
	result := make([]T, 0, l.size)
	start := l.tail.next
	current := start
	for {
		result = append(result, current.value)
		current = current.next
		if current == start {
			break
		}
	}
	return result
}

// Stack is a LIFO stack.
type Stack[T any] struct {
	items []T
}

// NewStack creates a new Stack.
func NewStack[T any]() *Stack[T] {
	return &Stack[T]{}
}

// Len returns the number of elements.
func (s *Stack[T]) Len() int { return len(s.items) }

// IsEmpty returns true if the stack is empty.
func (s *Stack[T]) IsEmpty() bool { return len(s.items) == 0 }

// Push adds a value.
func (s *Stack[T]) Push(value T) {
	s.items = append(s.items, value)
}

// Pop removes and returns the top value.
func (s *Stack[T]) Pop() (T, error) {
	var zero T
	if s.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	value := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return value, nil
}

// Peek returns the top value without removing it.
func (s *Stack[T]) Peek() (T, error) {
	var zero T
	if s.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	return s.items[len(s.items)-1], nil
}

// ToSlice returns a slice from top to bottom.
func (s *Stack[T]) ToSlice() []T {
	result := make([]T, len(s.items))
	for i := len(s.items) - 1; i >= 0; i-- {
		result[len(s.items)-1-i] = s.items[i]
	}
	return result
}

// qNode is a queue node.
type qNode[T any] struct {
	value T
	next  *qNode[T]
}

// Queue is a FIFO queue.
type Queue[T any] struct {
	front *qNode[T]
	rear  *qNode[T]
	size  int
}

// NewQueue creates a new Queue.
func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{}
}

// Len returns the number of elements.
func (q *Queue[T]) Len() int { return q.size }

// IsEmpty returns true if the queue is empty.
func (q *Queue[T]) IsEmpty() bool { return q.size == 0 }

// Enqueue adds a value.
func (q *Queue[T]) Enqueue(value T) {
	node := &qNode[T]{value: value}
	if q.rear == nil {
		q.front = node
		q.rear = node
	} else {
		q.rear.next = node
		q.rear = node
	}
	q.size++
}

// Dequeue removes and returns the front value.
func (q *Queue[T]) Dequeue() (T, error) {
	var zero T
	if q.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	value := q.front.value
	q.front = q.front.next
	if q.front == nil {
		q.rear = nil
	}
	q.size--
	return value, nil
}

// Peek returns the front value without removing it.
func (q *Queue[T]) Peek() (T, error) {
	var zero T
	if q.IsEmpty() {
		return zero, ErrEmptyStructure
	}
	return q.front.value, nil
}

// ToSlice returns a slice of all values.
func (q *Queue[T]) ToSlice() []T {
	result := make([]T, 0, q.size)
	current := q.front
	for current != nil {
		result = append(result, current.value)
		current = current.next
	}
	return result
}


