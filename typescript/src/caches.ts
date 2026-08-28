/**
 * A least-recently-used cache with a fixed capacity.
 */
export class LRUCache<K, V> {
  private _cache = new Map<K, V>();

  constructor(private _capacity: number) {
    if (_capacity <= 0) throw new Error('capacity must be positive');
  }

  get size(): number {
    return this._cache.size;
  }

  has(key: K): boolean {
    return this._cache.has(key);
  }

  get(key: K): V {
    if (!this._cache.has(key)) {
      throw new Error(`Key not found: ${String(key)}`);
    }
    const value = this._cache.get(key)!;
    this._cache.delete(key);
    this._cache.set(key, value);
    return value;
  }

  set(key: K, value: V): void {
    if (this._cache.has(key)) {
      this._cache.delete(key);
    } else if (this._cache.size >= this._capacity) {
      const oldest = this._cache.keys().next().value as K;
      this._cache.delete(oldest);
    }
    this._cache.set(key, value);
  }

  toString(): string {
    const obj: Record<string, V> = {};
    for (const [k, v] of this._cache) {
      obj[String(k)] = v;
    }
    return `LRUCache(${JSON.stringify(obj)})`;
  }
}
