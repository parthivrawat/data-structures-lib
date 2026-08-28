/**
 * A graph represented as an adjacency list.
 */
export class AdjacencyListGraph<T> {
  private _adj = new Map<T, T[]>();

  constructor(private _directed: boolean = false) {}

  get vertexCount(): number {
    return this._adj.size;
  }

  isEmpty(): boolean {
    return this._adj.size === 0;
  }

  addVertex(vertex: T): void {
    if (!this._adj.has(vertex)) {
      this._adj.set(vertex, []);
    }
  }

  removeVertex(vertex: T): void {
    if (!this._adj.has(vertex)) {
      throw new Error(`${vertex} not in graph`);
    }
    for (const [, neighbors] of this._adj) {
      const index = neighbors.indexOf(vertex);
      if (index !== -1) {
        neighbors.splice(index, 1);
      }
    }
    this._adj.delete(vertex);
  }

  addEdge(u: T, v: T): void {
    this.addVertex(u);
    this.addVertex(v);
    const uNeighbors = this._adj.get(u)!;
    if (!uNeighbors.includes(v)) {
      uNeighbors.push(v);
    }
    if (!this._directed) {
      const vNeighbors = this._adj.get(v)!;
      if (!vNeighbors.includes(u)) {
        vNeighbors.push(u);
      }
    }
  }

  removeEdge(u: T, v: T): void {
    const uNeighbors = this._adj.get(u);
    if (uNeighbors) {
      const index = uNeighbors.indexOf(v);
      if (index !== -1) uNeighbors.splice(index, 1);
    }
    if (!this._directed) {
      const vNeighbors = this._adj.get(v);
      if (vNeighbors) {
        const index = vNeighbors.indexOf(u);
        if (index !== -1) vNeighbors.splice(index, 1);
      }
    }
  }

  hasEdge(u: T, v: T): boolean {
    const neighbors = this._adj.get(u);
    return neighbors !== undefined && neighbors.includes(v);
  }

  neighbors(vertex: T): T[] {
    const n = this._adj.get(vertex);
    return n === undefined ? [] : [...n];
  }

  bfs(start: T): T[] {
    if (!this._adj.has(start)) return [];
    const visited = new Set<T>([start]);
    const queue: T[] = [start];
    const result: T[] = [];
    while (queue.length > 0) {
      const current = queue.shift()!;
      result.push(current);
      for (const neighbor of this._adj.get(current)!) {
        if (!visited.has(neighbor)) {
          visited.add(neighbor);
          queue.push(neighbor);
        }
      }
    }
    return result;
  }

  dfs(start: T): T[] {
    if (!this._adj.has(start)) return [];
    const visited = new Set<T>();
    const result: T[] = [];
    this._dfs(start, visited, result);
    return result;
  }

  private _dfs(vertex: T, visited: Set<T>, result: T[]): void {
    visited.add(vertex);
    result.push(vertex);
    for (const neighbor of this._adj.get(vertex)!) {
      if (!visited.has(neighbor)) {
        this._dfs(neighbor, visited, result);
      }
    }
  }

  has(vertex: T): boolean {
    return this._adj.has(vertex);
  }

  toString(): string {
    const obj: Record<string, T[]> = {};
    for (const [key, value] of this._adj) {
      obj[String(key)] = [...value];
    }
    return `AdjacencyListGraph(${JSON.stringify(obj)})`;
  }
}

/**
 * A graph represented as an adjacency matrix.
 */
export class AdjacencyMatrixGraph<T> {
  private _vertices: T[] = [];
  private _index = new Map<T, number>();
  private _matrix: number[][] = [];

  constructor(private _directed: boolean = false) {}

  get vertexCount(): number {
    return this._vertices.length;
  }

  isEmpty(): boolean {
    return this._vertices.length === 0;
  }

  addVertex(vertex: T): void {
    if (this._index.has(vertex)) return;
    this._index.set(vertex, this._vertices.length);
    this._vertices.push(vertex);
    for (const row of this._matrix) {
      row.push(0);
    }
    this._matrix.push(new Array(this._vertices.length).fill(0));
  }

  removeVertex(vertex: T): void {
    if (!this._index.has(vertex)) {
      throw new Error(`${vertex} not in graph`);
    }
    const index = this._index.get(vertex)!;
    this._vertices.splice(index, 1);
    this._index.delete(vertex);
    this._matrix.splice(index, 1);
    for (const row of this._matrix) {
      row.splice(index, 1);
    }
    for (let i = 0; i < this._vertices.length; i++) {
      this._index.set(this._vertices[i], i);
    }
  }

  addEdge(u: T, v: T, weight: number = 1): void {
    this.addVertex(u);
    this.addVertex(v);
    this._matrix[this._index.get(u)!][this._index.get(v)!] = weight;
    if (!this._directed) {
      this._matrix[this._index.get(v)!][this._index.get(u)!] = weight;
    }
  }

  removeEdge(u: T, v: T): void {
    if (this._index.has(u) && this._index.has(v)) {
      this._matrix[this._index.get(u)!][this._index.get(v)!] = 0;
      if (!this._directed) {
        this._matrix[this._index.get(v)!][this._index.get(u)!] = 0;
      }
    }
  }

  hasEdge(u: T, v: T): boolean {
    return this._index.has(u) && this._index.has(v) && this._matrix[this._index.get(u)!][this._index.get(v)!] !== 0;
  }

  weight(u: T, v: T): number {
    if (!this._index.has(u) || !this._index.has(v)) {
      throw new Error('vertex not in graph');
    }
    return this._matrix[this._index.get(u)!][this._index.get(v)!];
  }

  neighbors(vertex: T): T[] {
    if (!this._index.has(vertex)) return [];
    const index = this._index.get(vertex)!;
    const result: T[] = [];
    for (let i = 0; i < this._matrix[index].length; i++) {
      if (this._matrix[index][i] !== 0) {
        result.push(this._vertices[i]);
      }
    }
    return result;
  }

  bfs(start: T): T[] {
    if (!this._index.has(start)) return [];
    const visited = new Set<T>([start]);
    const queue: T[] = [start];
    const result: T[] = [];
    while (queue.length > 0) {
      const current = queue.shift()!;
      result.push(current);
      for (const neighbor of this.neighbors(current)) {
        if (!visited.has(neighbor)) {
          visited.add(neighbor);
          queue.push(neighbor);
        }
      }
    }
    return result;
  }

  dfs(start: T): T[] {
    if (!this._index.has(start)) return [];
    const visited = new Set<T>();
    const result: T[] = [];
    this._dfs(start, visited, result);
    return result;
  }

  private _dfs(vertex: T, visited: Set<T>, result: T[]): void {
    visited.add(vertex);
    result.push(vertex);
    for (const neighbor of this.neighbors(vertex)) {
      if (!visited.has(neighbor)) {
        this._dfs(neighbor, visited, result);
      }
    }
  }

  has(vertex: T): boolean {
    return this._index.has(vertex);
  }

  toString(): string {
    return `AdjacencyMatrixGraph(vertices=${JSON.stringify(this._vertices)}, matrix=${JSON.stringify(this._matrix)})`;
  }
}
