function defaultCompare<T>(a: T, b: T): number {
  if (a < b) return -1;
  if (a > b) return 1;
  return 0;
}

class BSTNode<T> {
  value: T;
  left: BSTNode<T> | null = null;
  right: BSTNode<T> | null = null;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A binary search tree.
 */
export class BinarySearchTree<T> {
  private _root: BSTNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>, private _compare: (a: T, b: T) => number = defaultCompare) {
    if (iterable) {
      for (const value of iterable) {
        this.insert(value);
      }
    }
  }

  get size(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  insert(value: T): void {
    if (this.search(value)) return;
    this._root = this._insert(this._root, value);
    this._size++;
  }

  private _insert(node: BSTNode<T> | null, value: T): BSTNode<T> {
    if (node === null) return new BSTNode(value);
    const cmp = this._compare(value, node.value);
    if (cmp < 0) node.left = this._insert(node.left, value);
    else if (cmp > 0) node.right = this._insert(node.right, value);
    return node;
  }

  delete(value: T): void {
    if (!this.search(value)) throw new Error(`${value} not in tree`);
    this._root = this._delete(this._root, value);
    this._size--;
  }

  private _delete(node: BSTNode<T> | null, value: T): BSTNode<T> | null {
    if (node === null) return null;
    const cmp = this._compare(value, node.value);
    if (cmp < 0) {
      node.left = this._delete(node.left, value);
    } else if (cmp > 0) {
      node.right = this._delete(node.right, value);
    } else {
      if (node.left === null) return node.right;
      if (node.right === null) return node.left;
      const successor = this._minNode(node.right);
      node.value = successor.value;
      node.right = this._delete(node.right, successor.value);
    }
    return node;
  }

  private _minNode(node: BSTNode<T>): BSTNode<T> {
    while (node.left !== null) node = node.left;
    return node;
  }

  search(value: T): boolean {
    let node = this._root;
    while (node !== null) {
      const cmp = this._compare(value, node.value);
      if (cmp === 0) return true;
      node = cmp < 0 ? node.left : node.right;
    }
    return false;
  }

  has(value: T): boolean {
    return this.search(value);
  }

  *[Symbol.iterator](): IterableIterator<T> {
    yield* this._inorder(this._root);
  }

  private *_inorder(node: BSTNode<T> | null): IterableIterator<T> {
    if (node !== null) {
      yield* this._inorder(node.left);
      yield node.value;
      yield* this._inorder(node.right);
    }
  }

  toString(): string {
    return `BinarySearchTree([${Array.from(this).join(', ')}])`;
  }
}

class AVLNode<T> {
  value: T;
  left: AVLNode<T> | null = null;
  right: AVLNode<T> | null = null;
  height = 1;

  constructor(value: T) {
    this.value = value;
  }
}

/**
 * A self-balancing AVL tree.
 */
export class AVLTree<T> {
  private _root: AVLNode<T> | null = null;
  private _size = 0;

  constructor(iterable?: Iterable<T>, private _compare: (a: T, b: T) => number = defaultCompare) {
    if (iterable) {
      for (const value of iterable) {
        this.insert(value);
      }
    }
  }

  get size(): number {
    return this._size;
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  height(): number {
    return this._height(this._root);
  }

  private _height(node: AVLNode<T> | null): number {
    return node === null ? 0 : node.height;
  }

  private _updateHeight(node: AVLNode<T>): void {
    node.height = 1 + Math.max(this._height(node.left), this._height(node.right));
  }

  private _balanceFactor(node: AVLNode<T>): number {
    return this._height(node.left) - this._height(node.right);
  }

  private _rightRotate(y: AVLNode<T>): AVLNode<T> {
    const x = y.left!;
    const t2 = x.right;
    x.right = y;
    y.left = t2;
    this._updateHeight(y);
    this._updateHeight(x);
    return x;
  }

  private _leftRotate(x: AVLNode<T>): AVLNode<T> {
    const y = x.right!;
    const t2 = y.left;
    y.left = x;
    x.right = t2;
    this._updateHeight(x);
    this._updateHeight(y);
    return y;
  }

  insert(value: T): void {
    if (this.search(value)) return;
    this._root = this._insert(this._root, value);
    this._size++;
  }

  private _insert(node: AVLNode<T> | null, value: T): AVLNode<T> {
    if (node === null) return new AVLNode(value);
    const cmp = this._compare(value, node.value);
    if (cmp < 0) node.left = this._insert(node.left, value);
    else if (cmp > 0) node.right = this._insert(node.right, value);
    else return node;

    this._updateHeight(node);
    const balance = this._balanceFactor(node);

    if (balance > 1 && this._compare(value, node.left!.value) < 0) {
      return this._rightRotate(node);
    }
    if (balance < -1 && this._compare(value, node.right!.value) > 0) {
      return this._leftRotate(node);
    }
    if (balance > 1 && this._compare(value, node.left!.value) > 0) {
      node.left = this._leftRotate(node.left!);
      return this._rightRotate(node);
    }
    if (balance < -1 && this._compare(value, node.right!.value) < 0) {
      node.right = this._rightRotate(node.right!);
      return this._leftRotate(node);
    }
    return node;
  }

  delete(value: T): void {
    if (!this.search(value)) throw new Error(`${value} not in tree`);
    this._root = this._delete(this._root, value);
    this._size--;
  }

  private _delete(node: AVLNode<T> | null, value: T): AVLNode<T> | null {
    if (node === null) return null;
    const cmp = this._compare(value, node.value);
    if (cmp < 0) {
      node.left = this._delete(node.left, value);
    } else if (cmp > 0) {
      node.right = this._delete(node.right, value);
    } else {
      if (node.left === null || node.right === null) {
        const temp = node.left !== null ? node.left : node.right;
        if (temp === null) return null;
        return temp;
      }
      const successor = this._minValueNode(node.right);
      node.value = successor.value;
      node.right = this._delete(node.right, successor.value);
    }

    if (node === null) return null;

    this._updateHeight(node);
    const balance = this._balanceFactor(node);

    if (balance > 1 && this._balanceFactor(node.left!) >= 0) {
      return this._rightRotate(node);
    }
    if (balance > 1 && this._balanceFactor(node.left!) < 0) {
      node.left = this._leftRotate(node.left!);
      return this._rightRotate(node);
    }
    if (balance < -1 && this._balanceFactor(node.right!) <= 0) {
      return this._leftRotate(node);
    }
    if (balance < -1 && this._balanceFactor(node.right!) > 0) {
      node.right = this._rightRotate(node.right!);
      return this._leftRotate(node);
    }
    return node;
  }

  private _minValueNode(node: AVLNode<T>): AVLNode<T> {
    while (node.left !== null) node = node.left;
    return node;
  }

  search(value: T): boolean {
    let node = this._root;
    while (node !== null) {
      const cmp = this._compare(value, node.value);
      if (cmp === 0) return true;
      node = cmp < 0 ? node.left : node.right;
    }
    return false;
  }

  has(value: T): boolean {
    return this.search(value);
  }

  *[Symbol.iterator](): IterableIterator<T> {
    yield* this._inorder(this._root);
  }

  private *_inorder(node: AVLNode<T> | null): IterableIterator<T> {
    if (node !== null) {
      yield* this._inorder(node.left);
      yield node.value;
      yield* this._inorder(node.right);
    }
  }

  toString(): string {
    return `AVLTree([${Array.from(this).join(', ')}])`;
  }
}
