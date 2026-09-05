/**
 * Compute a non-negative integer hash for an arbitrary value.
 */
import { InvalidArgumentError, NotFoundError } from './exceptions';

export function defaultHash(value: unknown): number {
  if (typeof value === 'number') {
    return Math.abs(Math.floor(value));
  }
  if (typeof value === 'bigint') {
    return Math.abs(Number(value));
  }
  let str: string;
  if (typeof value === 'string') {
    str = value;
  } else {
    try {
      str = JSON.stringify(value);
    } catch {
      str = String(value);
    }
  }
  let h = 0;
  for (let i = 0; i < str.length; i++) {
    h = (h << 5) - h + str.charCodeAt(i);
    h = h | 0;
  }
  return Math.abs(h);
}

/**
 * A hash map using separate chaining.
 */
export class HashMap<K, V> {
  private _capacity: number;
  private _loadFactor: number;
  private _size: number;
  private _table: [K, V][][];

  constructor(capacity: number = 16, loadFactor: number = 0.75) {
    if (capacity <= 0) throw new InvalidArgumentError('capacity must be positive');
    if (loadFactor <= 0 || loadFactor > 1) throw new InvalidArgumentError('loadFactor must be between 0 and 1');
    this._capacity = capacity;
    this._loadFactor = loadFactor;
    this._size = 0;
    this._table = Array.from({ length: capacity }, () => []);
  }

  private _index(key: K): number {
    return defaultHash(key) % this._capacity;
  }

  set(key: K, value: V): void {
    const bucket = this._table[this._index(key)];
    for (const entry of bucket) {
      if (entry[0] === key) {
        entry[1] = value;
        return;
      }
    }
    bucket.push([key, value]);
    this._size++;
    if (this._size / this._capacity > this._loadFactor) {
      this._resize();
    }
  }

  get(key: K): V | undefined {
    const bucket = this._table[this._index(key)];
    for (const [k, v] of bucket) {
      if (k === key) return v;
    }
    return undefined;
  }

  has(key: K): boolean {
    const bucket = this._table[this._index(key)];
    for (const [k] of bucket) {
      if (k === key) return true;
    }
    return false;
  }

  delete(key: K): boolean {
    const bucket = this._table[this._index(key)];
    for (let i = 0; i < bucket.length; i++) {
      if (bucket[i][0] === key) {
        bucket.splice(i, 1);
        this._size--;
        return true;
      }
    }
    return false;
  }

  keys(): K[] {
    const result: K[] = [];
    for (const bucket of this._table) {
      for (const [k] of bucket) result.push(k);
    }
    return result;
  }

  values(): V[] {
    const result: V[] = [];
    for (const bucket of this._table) {
      for (const [, v] of bucket) result.push(v);
    }
    return result;
  }

  entries(): [K, V][] {
    const result: [K, V][] = [];
    for (const bucket of this._table) {
      for (const entry of bucket) result.push([...entry]);
    }
    return result;
  }

  get size(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  *[Symbol.iterator](): IterableIterator<K> {
    for (const bucket of this._table) {
      for (const [k] of bucket) {
        yield k;
      }
    }
  }

  toString(): string {
    const obj: Record<string, V> = {};
    for (const [k, v] of this.entries()) {
      obj[String(k)] = v;
    }
    return `HashMap(${JSON.stringify(obj)})`;
  }

  private _resize(): void {
    const oldTable = this._table;
    this._capacity *= 2;
    this._size = 0;
    this._table = Array.from({ length: this._capacity }, () => []);
    for (const bucket of oldTable) {
      for (const [k, v] of bucket) {
        this.set(k, v);
      }
    }
  }
}

/**
 * A set implemented with a hash map.
 */
export class HashSet<T> {
  private _map = new HashMap<T, true>();

  constructor(iterable?: Iterable<T>) {
    if (iterable) {
      for (const value of iterable) {
        this.add(value);
      }
    }
  }

  add(value: T): void {
    this._map.set(value, true);
  }

  remove(value: T): void {
    if (!this.has(value)) {
      throw new NotFoundError(`${value} not in set`);
    }
    this._map.delete(value);
  }

  discard(value: T): void {
    this._map.delete(value);
  }

  has(value: T): boolean {
    return this._map.has(value);
  }

  get size(): number {
    return this._map.size;
  }

  isEmpty(): boolean {
    return this._map.isEmpty();
  }

  *[Symbol.iterator](): IterableIterator<T> {
    for (const key of this._map) {
      yield key;
    }
  }

  toString(): string {
    return `HashSet([${Array.from(this).join(', ')}])`;
  }
}
