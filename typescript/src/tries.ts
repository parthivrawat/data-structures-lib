import { InvalidArgumentError, NotFoundError } from './exceptions';

class TrieNode {
  children = new Map<string, TrieNode>();
  isEnd = false;
  count = 0;
}

/**
 * A prefix tree for string storage.
 */
export class Trie {
  private _root = new TrieNode();
  private _size = 0;

  constructor(words?: Iterable<string>) {
    if (words) {
      for (const word of words) {
        this.insert(word);
      }
    }
  }

  get size(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  insert(word: string): void {
    if (typeof word !== 'string') throw new InvalidArgumentError('Trie only supports string keys');
    let node = this._root;
    for (const char of word) {
      if (!node.children.has(char)) {
        node.children.set(char, new TrieNode());
      }
      node = node.children.get(char)!;
      node.count++;
    }
    if (!node.isEnd) {
      this._size++;
      node.isEnd = true;
    }
  }

  private _find(word: string): TrieNode | null {
    let node = this._root;
    for (const char of word) {
      if (!node.children.has(char)) return null;
      node = node.children.get(char)!;
    }
    return node;
  }

  search(word: string): boolean {
    if (typeof word !== 'string') throw new InvalidArgumentError('Trie only supports string keys');
    const node = this._find(word);
    return node !== null && node.isEnd;
  }

  startsWith(prefix: string): boolean {
    if (typeof prefix !== 'string') throw new InvalidArgumentError('Trie only supports string keys');
    return this._find(prefix) !== null;
  }

  delete(word: string): void {
    if (!this.search(word)) throw new NotFoundError(`${word} not in trie`);
    this._delete(this._root, word, 0);
    this._size--;
  }

  private _delete(node: TrieNode, word: string, index: number): void {
    if (index === word.length) {
      node.isEnd = false;
      return;
    }
    const char = word[index];
    const child = node.children.get(char)!;
    this._delete(child, word, index + 1);
    child.count--;
    if (child.count === 0) {
      node.children.delete(char);
    }
  }

  words(prefix: string = ''): string[] {
    const node = prefix ? this._find(prefix) : this._root;
    if (node === null) return [];
    const result: string[] = [];
    this._collect(node, prefix.split(''), result);
    return result;
  }

  private _collect(node: TrieNode, prefix: string[], result: string[]): void {
    if (node.isEnd) result.push(prefix.join(''));
    for (const [char, child] of node.children) {
      prefix.push(char);
      this._collect(child, prefix, result);
      prefix.pop();
    }
  }

  *[Symbol.iterator](): IterableIterator<string> {
    for (const word of this.words()) {
      yield word;
    }
  }

  toString(): string {
    return `Trie([${this.words().join(', ')}])`;
  }
}
