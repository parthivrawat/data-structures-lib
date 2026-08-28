import { EmptyStructureError } from './exceptions';

/**
 * A generic binary heap.
 */
export class Heap<T> {
  protected _data: T[] = [];

  constructor(
    private _isPrior: (a: T, b: T) => boolean,
    iterable?: Iterable<T>
  ) {
    if (iterable) {
      for (const value of iterable) {
        this.push(value);
      }
    }
  }

  get length(): number {
    return this._data.length;
  }

  isEmpty(): boolean {
    return this._data.length === 0;
  }

  push(value: T): void {
    this._data.push(value);
    this._siftUp(this._data.length - 1);
  }

  private _siftUp(index: number): void {
    let current = index;
    while (current > 0) {
      const parent = Math.floor((current - 1) / 2);
      if (this._isPrior(this._data[current], this._data[parent])) {
        [this._data[current], this._data[parent]] = [this._data[parent], this._data[current]];
        current = parent;
      } else {
        break;
      }
    }
  }

  pop(): T {
    if (this.isEmpty()) {
      throw new EmptyStructureError('pop from empty heap');
    }
    if (this._data.length === 1) {
      return this._data.pop() as T;
    }
    const root = this._data[0];
    this._data[0] = this._data.pop() as T;
    this._siftDown(0);
    return root;
  }

  private _siftDown(index: number): void {
    const length = this._data.length;
    let current = index;
    while (true) {
      let target = current;
      const left = 2 * current + 1;
      const right = 2 * current + 2;
      if (left < length && this._isPrior(this._data[left], this._data[target])) {
        target = left;
      }
      if (right < length && this._isPrior(this._data[right], this._data[target])) {
        target = right;
      }
      if (target === current) break;
      [this._data[current], this._data[target]] = [this._data[target], this._data[current]];
      current = target;
    }
  }

  peek(): T {
    if (this.isEmpty()) {
      throw new EmptyStructureError('peek from empty heap');
    }
    return this._data[0];
  }

  *[Symbol.iterator](): IterableIterator<T> {
    for (const value of this._data) {
      yield value;
    }
  }

  toString(): string {
    return `Heap([${this._data.join(', ')}])`;
  }
}

/**
 * A min-heap.
 */
export class MinHeap<T> extends Heap<T> {
  constructor(iterable?: Iterable<T>) {
    super((a, b) => a < b, iterable);
  }

  toString(): string {
    return `MinHeap([${this._data.join(', ')}])`;
  }
}

/**
 * A max-heap.
 */
export class MaxHeap<T> extends Heap<T> {
  constructor(iterable?: Iterable<T>) {
    super((a, b) => a > b, iterable);
  }

  toString(): string {
    return `MaxHeap([${this._data.join(', ')}])`;
  }
}
