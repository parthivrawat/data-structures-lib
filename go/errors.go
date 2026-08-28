package datastructures

import "errors"

// Common errors used by the data structures in this package.
var (
	ErrEmptyStructure  = errors.New("structure is empty")
	ErrNotFound        = errors.New("not found")
	ErrIndexOutOfRange = errors.New("index out of range")
	ErrInvalidCapacity = errors.New("capacity must be positive")
	ErrKeyNotFound     = errors.New("key not found")
)
