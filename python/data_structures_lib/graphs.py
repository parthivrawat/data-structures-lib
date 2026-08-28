'''Graph data structures.'''

from collections import deque
from typing import Any, Dict, List, Optional, Set

__all__ = ['AdjacencyListGraph', 'AdjacencyMatrixGraph']


class AdjacencyListGraph:
    '''A graph represented as an adjacency list.'''

    def __init__(self, directed: bool = False) -> None:
        self._adj: Dict[Any, List[Any]] = {}
        self._directed = directed

    def __len__(self) -> int:
        return len(self._adj)

    def is_empty(self) -> bool:
        return len(self._adj) == 0

    def add_vertex(self, vertex: Any) -> None:
        '''Add a vertex to the graph.'''
        if vertex not in self._adj:
            self._adj[vertex] = []

    def remove_vertex(self, vertex: Any) -> None:
        '''Remove a vertex and all its edges.'''
        if vertex not in self._adj:
            raise ValueError(f'{vertex!r} not in graph')
        for u in list(self._adj):
            if vertex in self._adj[u]:
                self._adj[u].remove(vertex)
        del self._adj[vertex]

    def add_edge(self, u: Any, v: Any) -> None:
        '''Add an undirected or directed edge.'''
        self.add_vertex(u)
        self.add_vertex(v)
        if v not in self._adj[u]:
            self._adj[u].append(v)
        if not self._directed:
            if u not in self._adj[v]:
                self._adj[v].append(u)

    def remove_edge(self, u: Any, v: Any) -> None:
        '''Remove an edge.'''
        if u in self._adj and v in self._adj[u]:
            self._adj[u].remove(v)
            if not self._directed:
                self._adj[v].remove(u)

    def has_edge(self, u: Any, v: Any) -> bool:
        '''Return True if there is an edge from u to v.'''
        return u in self._adj and v in self._adj[u]

    def neighbors(self, vertex: Any) -> List[Any]:
        '''Return neighbors of vertex.'''
        return list(self._adj.get(vertex, []))

    def bfs(self, start: Any) -> List[Any]:
        '''Return vertices in breadth-first order.'''
        if start not in self._adj:
            return []
        visited: Set[Any] = {start}
        queue = deque([start])
        result: List[Any] = []
        while queue:
            current = queue.popleft()
            result.append(current)
            for neighbor in self._adj[current]:
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
        return result

    def dfs(self, start: Any) -> List[Any]:
        '''Return vertices in depth-first order.'''
        if start not in self._adj:
            return []
        visited: Set[Any] = set()
        result: List[Any] = []
        self._dfs(start, visited, result)
        return result

    def _dfs(self, vertex: Any, visited: Set[Any], result: List[Any]) -> None:
        visited.add(vertex)
        result.append(vertex)
        for neighbor in self._adj[vertex]:
            if neighbor not in visited:
                self._dfs(neighbor, visited, result)

    def __contains__(self, vertex: Any) -> bool:
        return vertex in self._adj

    def __repr__(self) -> str:
        return f'AdjacencyListGraph({dict(self._adj)})'


class AdjacencyMatrixGraph:
    '''A graph represented as an adjacency matrix.'''

    def __init__(self, directed: bool = False) -> None:
        self._vertices: List[Any] = []
        self._index: Dict[Any, int] = {}
        self._matrix: List[List[int]] = []
        self._directed = directed

    def __len__(self) -> int:
        return len(self._vertices)

    def is_empty(self) -> bool:
        return len(self._vertices) == 0

    def add_vertex(self, vertex: Any) -> None:
        '''Add a vertex.'''
        if vertex in self._index:
            return
        self._index[vertex] = len(self._vertices)
        self._vertices.append(vertex)
        for row in self._matrix:
            row.append(0)
        self._matrix.append([0] * len(self._vertices))

    def remove_vertex(self, vertex: Any) -> None:
        '''Remove a vertex.'''
        if vertex not in self._index:
            raise ValueError(f'{vertex!r} not in graph')
        index = self._index[vertex]
        self._vertices.pop(index)
        del self._index[vertex]
        self._matrix.pop(index)
        for row in self._matrix:
            row.pop(index)
        for i, v in enumerate(self._vertices):
            self._index[v] = i

    def add_edge(self, u: Any, v: Any, weight: int = 1) -> None:
        '''Add an edge with weight.'''
        self.add_vertex(u)
        self.add_vertex(v)
        self._matrix[self._index[u]][self._index[v]] = weight
        if not self._directed:
            self._matrix[self._index[v]][self._index[u]] = weight

    def remove_edge(self, u: Any, v: Any) -> None:
        '''Remove an edge.'''
        if u in self._index and v in self._index:
            self._matrix[self._index[u]][self._index[v]] = 0
            if not self._directed:
                self._matrix[self._index[v]][self._index[u]] = 0

    def has_edge(self, u: Any, v: Any) -> bool:
        '''Return True if there is an edge from u to v.'''
        return u in self._index and v in self._index and self._matrix[self._index[u]][self._index[v]] != 0

    def weight(self, u: Any, v: Any) -> int:
        '''Return the weight of the edge from u to v.'''
        if u not in self._index or v not in self._index:
            raise ValueError('vertex not in graph')
        return self._matrix[self._index[u]][self._index[v]]

    def neighbors(self, vertex: Any) -> List[Any]:
        '''Return neighbors of vertex.'''
        if vertex not in self._index:
            return []
        index = self._index[vertex]
        result: List[Any] = []
        for i, w in enumerate(self._matrix[index]):
            if w != 0:
                result.append(self._vertices[i])
        return result

    def bfs(self, start: Any) -> List[Any]:
        '''Return vertices in breadth-first order.'''
        if start not in self._index:
            return []
        visited: Set[Any] = {start}
        queue = deque([start])
        result: List[Any] = []
        while queue:
            current = queue.popleft()
            result.append(current)
            for neighbor in self.neighbors(current):
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
        return result

    def dfs(self, start: Any) -> List[Any]:
        '''Return vertices in depth-first order.'''
        if start not in self._index:
            return []
        visited: Set[Any] = set()
        result: List[Any] = []
        self._dfs(start, visited, result)
        return result

    def _dfs(self, vertex: Any, visited: Set[Any], result: List[Any]) -> None:
        visited.add(vertex)
        result.append(vertex)
        for neighbor in self.neighbors(vertex):
            if neighbor not in visited:
                self._dfs(neighbor, visited, result)

    def __contains__(self, vertex: Any) -> bool:
        return vertex in self._index

    def __repr__(self) -> str:
        return f'AdjacencyMatrixGraph(vertices={self._vertices}, matrix={self._matrix})'
