import {
  EmptyStructureError,
  IndexOutOfRangeError,
  InvalidArgumentError,
  NotFoundError,
} from './exceptions';

/**
 * A dynamic array with automatic resizing.
 */
export class DynamicArray<T> {
  private _data: (T | undefined)[];
  private _size: number;
  private _capacity: number;

  constructor(initialCapacity: number = 10) {
    if (initialCapacity <= 0) {
      throw new InvalidArgumentError('initialCapacity must be positive');
    }
    this._capacity = initialCapacity;
    this._size = 0;
    this._data = new Array(initialCapacity);
  }

  get length(): number {
    return this._size;
  }

  get capacity(): number {
    return this._capacity;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  append(value: T): void {
    if (this._size === this._capacity) {
      this._resize(2 * this._capacity);
    }
    this._data[this._size] = value;
    this._size++;
  }

  insert(index: number, value: T): void {
    if (index < 0 || index > this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    if (this._size === this._capacity) {
      this._resize(2 * this._capacity);
    }
    for (let i = this._size; i > index; i--) {
      this._data[i] = this._data[i - 1];
    }
    this._data[index] = value;
    this._size++;
  }

  remove(value: T): void {
    for (let i = 0; i < this._size; i++) {
      if (this._data[i] === value) {
        this.pop(i);
        return;
      }
    }
    throw new NotFoundError(`${value} not in array`);
  }

  pop(index: number = -1): T {
    if (this.isEmpty()) {
      throw new EmptyStructureError('pop from empty array');
    }
    let idx = index;
    if (idx < 0) idx += this._size;
    if (idx < 0 || idx >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    const value = this._data[idx] as T;
    for (let i = idx; i < this._size - 1; i++) {
      this._data[i] = this._data[i + 1];
    }
    this._size--;
    if (this._size > 0 && this._size === Math.floor(this._capacity / 4) && this._capacity > 10) {
      this._resize(Math.floor(this._capacity / 2));
    }
    return value;
  }

  get(index: number): T {
    if (index < 0 || index >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    return this._data[index] as T;
  }

  set(index: number, value: T): void {
    if (index < 0 || index >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    this._data[index] = value;
  }

  *[Symbol.iterator](): IterableIterator<T> {
    for (let i = 0; i < this._size; i++) {
      yield this._data[i] as T;
    }
  }

  toString(): string {
    return `DynamicArray([${Array.from(this).join(', ')}])`;
  }

  private _resize(newCapacity: number): void {
    const newData: (T | undefined)[] = new Array(newCapacity);
    for (let i = 0; i < this._size; i++) {
      newData[i] = this._data[i];
    }
    this._data = newData;
    this._capacity = newCapacity;
  }
}

class SNode<T> {
  value: T;
  next: SNode<T> | null = null;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A singly linked list.
 */
export class SinglyLinkedList<T> {
  private _head: SNode<T> | null = null;
  private _tail: SNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.append(value);
      }
    }
  }

  get length(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  private _nodeAt(index: number): SNode<T> {
    if (index < 0 || index >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    let current = this._head!;
    for (let i = 0; i < index; i++) {
      current = current.next!;
    }
    return current;
  }

  append(value: T): void {
    const node = new SNode(value);
    if (!this._tail) {
      this._head = this._tail = node;
    } else {
      this._tail.next = node;
      this._tail = node;
    }
    this._size++;
  }

  prepend(value: T): void {
    const node = new SNode(value);
    if (!this._head) {
      this._head = this._tail = node;
    } else {
      node.next = this._head;
      this._head = node;
    }
    this._size++;
  }

  insert(index: number, value: T): void {
    if (index < 0 || index > this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    if (index === 0) {
      this.prepend(value);
      return;
    }
    if (index === this._size) {
      this.append(value);
      return;
    }
    const node = new SNode(value);
    const prev = this._nodeAt(index - 1);
    node.next = prev.next;
    prev.next = node;
    this._size++;
  }

  remove(value: T): void {
    if (!this._head) throw new NotFoundError(`${value} not in list`);
    if (this._head.value === value) {
      this._head = this._head.next;
      if (!this._head) this._tail = null;
      this._size--;
      return;
    }
    let current = this._head;
    while (current.next) {
      if (current.next.value === value) {
        const removed = current.next;
        current.next = removed.next;
        if (removed === this._tail) this._tail = current;
        this._size--;
        return;
      }
      current = current.next;
    }
    throw new NotFoundError(`${value} not in list`);
  }

  pop(index: number = -1): T {
    if (this.isEmpty()) {
      throw new EmptyStructureError('pop from empty list');
    }
    let idx = index;
    if (idx < 0) idx += this._size;
    if (idx < 0 || idx >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    if (idx === 0) {
      const value = this._head!.value;
      this._head = this._head!.next;
      if (!this._head) this._tail = null;
      this._size--;
      return value;
    }
    const prev = this._nodeAt(idx - 1);
    const node = prev.next!;
    prev.next = node.next;
    if (node === this._tail) this._tail = prev;
    this._size--;
    return node.value;
  }

  find(value: T): number {
    let current = this._head;
    let i = 0;
    while (current) {
      if (current.value === value) return i;
      current = current.next;
      i++;
    }
    throw new NotFoundError(`${value} not in list`);
  }

  get(index: number): T {
    return this._nodeAt(index).value;
  }

  set(index: number, value: T): void {
    this._nodeAt(index).value = value;
  }

  *[Symbol.iterator](): IterableIterator<T> {
    let current = this._head;
    while (current) {
      yield current.value;
      current = current.next;
    }
  }

  toString(): string {
    return `SinglyLinkedList([${Array.from(this).join(', ')}])`;
  }
}

class DNode<T> {
  value: T;
  prev: DNode<T> | null = null;
  next: DNode<T> | null = null;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A doubly linked list.
 */
export class DoublyLinkedList<T> {
  private _head: DNode<T> | null = null;
  private _tail: DNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.append(value);
      }
    }
  }

  get length(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  private _nodeAt(index: number): DNode<T> {
    if (index < 0 || index >= this._size) {
      throw new IndexOutOfRangeError('index out of range');
    }
    if (index < this._size / 2) {
      let current = this._head!;
      for (let i = 0; i < index; i++) current = current.next!;
      return current;
    } else {
      let current = this._tail!;
      for (let i = this._size - 1; i > index; i--) current = current.prev!;
      return current;
    }
  }

  append(value: T): void {
    const node = new DNode(value);
    if (!this._tail) {
      this._head = this._tail = node;
    } else {
      node.prev = this._tail;
      this._tail.next = node;
      this._tail = node;
    }
    this._size++;
  }

  prepend(value: T): void {
    const node = new DNode(value);
    if (!this._head) {
      this._head = this._tail = node;
    } else {
      node.next = this._head;
      this._head.prev = node;
      this._head = node;
    }
    this._size++;
  }

  insert(index: number, value: T): void {
    if (index < 0 || index > this._size) throw new IndexOutOfRangeError('index out of range');
    if (index === 0) { this.prepend(value); return; }
    if (index === this._size) { this.append(value); return; }
    const next = this._nodeAt(index);
    const prev = next.prev!;
    const node = new DNode(value);
    node.prev = prev;
    node.next = next;
    prev.next = node;
    next.prev = node;
    this._size++;
  }

  remove(value: T): void {
    let current = this._head;
    while (current) {
      if (current.value === value) {
        this._removeNode(current);
        return;
      }
      current = current.next;
    }
    throw new NotFoundError(`${value} not in list`);
  }

  private _removeNode(node: DNode<T>): void {
    if (node.prev) node.prev.next = node.next;
    else this._head = node.next;
    if (node.next) node.next.prev = node.prev;
    else this._tail = node.prev;
    this._size--;
  }

  pop(index: number = -1): T {
    if (this.isEmpty()) throw new EmptyStructureError('pop from empty list');
    let idx = index;
    if (idx < 0) idx += this._size;
    if (idx < 0 || idx >= this._size) throw new IndexOutOfRangeError('index out of range');
    const node = this._nodeAt(idx);
    const value = node.value;
    this._removeNode(node);
    return value;
  }

  find(value: T): number {
    let current = this._head;
    let i = 0;
    while (current) {
      if (current.value === value) return i;
      current = current.next;
      i++;
    }
    throw new NotFoundError(`${value} not in list`);
  }

  get(index: number): T {
    return this._nodeAt(index).value;
  }

  set(index: number, value: T): void {
    this._nodeAt(index).value = value;
  }

  *[Symbol.iterator](): IterableIterator<T> {
    let current = this._head;
    while (current) {
      yield current.value;
      current = current.next;
    }
  }

  toString(): string {
    return `DoublyLinkedList([${Array.from(this).join(', ')}])`;
  }
}

class CNode<T> {
  value: T;
  next: CNode<T> | null = null;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A circular linked list.
 */
export class CircularLinkedList<T> {
  private _tail: CNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.append(value);
      }
    }
  }

  get length(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  append(value: T): void {
    const node = new CNode(value);
    if (!this._tail) {
      node.next = node;
      this._tail = node;
    } else {
      node.next = this._tail.next;
      this._tail.next = node;
      this._tail = node;
    }
    this._size++;
  }

  prepend(value: T): void {
    const node = new CNode(value);
    if (!this._tail) {
      node.next = node;
      this._tail = node;
    } else {
      node.next = this._tail.next;
      this._tail.next = node;
    }
    this._size++;
  }

  remove(value: T): void {
    if (this.isEmpty()) throw new NotFoundError(`${value} not in list`);
    const head = this._tail!.next!;
    if (head.value === value) {
      if (head === this._tail) {
        this._tail = null;
      } else {
        this._tail!.next = head.next;
      }
      this._size--;
      return;
    }
    let current = head;
    for (let i = 0; i < this._size - 1; i++) {
      if (current.next!.value === value) {
        current.next = current.next!.next;
        if (current.next === head) this._tail = current;
        this._size--;
        return;
      }
      current = current.next!;
    }
    throw new NotFoundError(`${value} not in list`);
  }

  *[Symbol.iterator](): IterableIterator<T> {
    if (!this._tail) return;
    const start = this._tail.next!;
    let current: CNode<T> = start;
    do {
      yield current.value;
      current = current.next!;
    } while (current !== start);
  }

  toString(): string {
    return `CircularLinkedList([${Array.from(this).join(', ')}])`;
  }
}

/**
 * A last-in, first-out (LIFO) stack.
 */
export class Stack<T> {
  private _items: T[] = [];

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.push(value);
      }
    }
  }

  get length(): number {
    return this._items.length;
  }

  isEmpty(): boolean {
    return this._items.length === 0;
  }

  push(value: T): void {
    this._items.push(value);
  }

  pop(): T {
    if (this.isEmpty()) throw new EmptyStructureError('pop from empty stack');
    return this._items.pop() as T;
  }

  peek(): T {
    if (this.isEmpty()) throw new EmptyStructureError('peek from empty stack');
    return this._items[this._items.length - 1];
  }

  *[Symbol.iterator](): IterableIterator<T> {
    for (let i = this._items.length - 1; i >= 0; i--) {
      yield this._items[i];
    }
  }

  toString(): string {
    return `Stack([${[...this].join(', ')}])`;
  }
}

class QNode<T> {
  value: T;
  next: QNode<T> | null = null;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A first-in, first-out (FIFO) queue.
 */
export class Queue<T> {
  private _front: QNode<T> | null = null;
  private _rear: QNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.enqueue(value);
      }
    }
  }

  get length(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  enqueue(value: T): void {
    const node = new QNode(value);
    if (!this._rear) {
      this._front = this._rear = node;
    } else {
      this._rear.next = node;
      this._rear = node;
    }
    this._size++;
  }

  dequeue(): T {
    if (this.isEmpty()) throw new EmptyStructureError('dequeue from empty queue');
    const value = this._front!.value;
    this._front = this._front!.next;
    if (!this._front) this._rear = null;
    this._size--;
    return value;
  }

  peek(): T {
    if (this.isEmpty()) throw new EmptyStructureError('peek from empty queue');
    return this._front!.value;
  }

  *[Symbol.iterator](): IterableIterator<T> {
    let current = this._front;
    while (current) {
      yield current.value;
      current = current.next;
    }
  }

  toString(): string {
    return `Queue([${Array.from(this).join(', ')}])`;
  }
}
