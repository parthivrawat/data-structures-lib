import { InvalidArgumentError } from './exceptions';

/**
 * A Bloom filter for membership queries.
 */
export class BloomFilter {
  private _size: number;
  private _hashCount: number;
  private _bits: Uint8Array;
  private _itemsAdded = 0;

  constructor(expectedItems: number = 1000, falsePositiveRate: number = 0.01) {
    if (expectedItems <= 0) throw new InvalidArgumentError('expectedItems must be positive');
    if (falsePositiveRate <= 0 || falsePositiveRate >= 1) throw new InvalidArgumentError('falsePositiveRate must be between 0 and 1');
    const m = Math.ceil(-(expectedItems * Math.log(falsePositiveRate)) / (Math.log(2) ** 2));
    const k = Math.max(1, Math.round((m / expectedItems) * Math.log(2)));
    this._size = m;
    this._hashCount = k;
    this._bits = new Uint8Array(Math.ceil(m / 8));
  }

  private *_hashes(item: unknown): IterableIterator<number> {
    const h1 = this._hash(item);
    const h2 = this._hash([item, 7]);
    for (let i = 0; i < this._hashCount; i++) {
      yield Math.abs((h1 + i * h2) % this._size);
    }
  }

  private _hash(value: unknown): number {
    const str = typeof value === 'string' ? value : JSON.stringify(value);
    let h = 0;
    for (let i = 0; i < str.length; i++) {
      h = (h << 5) - h + str.charCodeAt(i);
      h = h | 0;
    }
    return Math.abs(h);
  }

  add(item: unknown): void {
    for (const position of this._hashes(item)) {
      this._bits[Math.floor(position / 8)] |= 1 << (position % 8);
    }
    this._itemsAdded++;
  }

  has(item: unknown): boolean {
    for (const position of this._hashes(item)) {
      if ((this._bits[Math.floor(position / 8)] & (1 << (position % 8))) === 0) {
        return false;
      }
    }
    return true;
  }

  get count(): number {
    return this._itemsAdded;
  }

  expectedFpp(): number {
    return (1 - Math.exp(-this._hashCount * this._itemsAdded / this._size)) ** this._hashCount;
  }

  toString(): string {
    return `BloomFilter(size=${this._size}, hashCount=${this._hashCount}, itemsAdded=${this._itemsAdded})`;
  }
}
