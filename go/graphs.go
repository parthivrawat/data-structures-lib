package datastructures

// AdjacencyListGraph is a graph represented as an adjacency list.
type AdjacencyListGraph[T comparable] struct {
	adj     map[T][]T
	directed bool
}

// NewAdjacencyListGraph creates a new AdjacencyListGraph.
func NewAdjacencyListGraph[T comparable](directed bool) *AdjacencyListGraph[T] {
	return &AdjacencyListGraph[T]{
		adj:      make(map[T][]T),
		directed: directed,
	}
}

// VertexCount returns the number of vertices.
func (g *AdjacencyListGraph[T]) VertexCount() int { return len(g.adj) }

// IsEmpty returns true if the graph is empty.
func (g *AdjacencyListGraph[T]) IsEmpty() bool { return len(g.adj) == 0 }

// AddVertex adds a vertex.
func (g *AdjacencyListGraph[T]) AddVertex(vertex T) {
	if _, ok := g.adj[vertex]; !ok {
		g.adj[vertex] = nil
	}
}

// RemoveVertex removes a vertex and its edges.
func (g *AdjacencyListGraph[T]) RemoveVertex(vertex T) error {
	if _, ok := g.adj[vertex]; !ok {
		return ErrNotFound
	}
	for key := range g.adj {
		g.adj[key] = removeFromSlice(g.adj[key], vertex)
	}
	delete(g.adj, vertex)
	return nil
}

// AddEdge adds an edge.
func (g *AdjacencyListGraph[T]) AddEdge(u, v T) {
	g.AddVertex(u)
	g.AddVertex(v)
	if !contains(g.adj[u], v) {
		g.adj[u] = append(g.adj[u], v)
	}
	if !g.directed && !contains(g.adj[v], u) {
		g.adj[v] = append(g.adj[v], u)
	}
}

// RemoveEdge removes an edge.
func (g *AdjacencyListGraph[T]) RemoveEdge(u, v T) {
	g.adj[u] = removeFromSlice(g.adj[u], v)
	if !g.directed {
		g.adj[v] = removeFromSlice(g.adj[v], u)
	}
}

// HasEdge returns true if there is an edge from u to v.
func (g *AdjacencyListGraph[T]) HasEdge(u, v T) bool {
	return contains(g.adj[u], v)
}

// Neighbors returns the neighbors of a vertex.
func (g *AdjacencyListGraph[T]) Neighbors(vertex T) []T {
	if neighbors, ok := g.adj[vertex]; ok {
		result := make([]T, len(neighbors))
		copy(result, neighbors)
		return result
	}
	return nil
}

// BFS returns vertices in breadth-first order.
func (g *AdjacencyListGraph[T]) BFS(start T) []T {
	if _, ok := g.adj[start]; !ok {
		return nil
	}
	visited := make(map[T]bool)
	visited[start] = true
	queue := []T{start}
	var result []T
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		result = append(result, current)
		for _, neighbor := range g.adj[current] {
			if !visited[neighbor] {
				visited[neighbor] = true
				queue = append(queue, neighbor)
			}
		}
	}
	return result
}

// DFS returns vertices in depth-first order.
func (g *AdjacencyListGraph[T]) DFS(start T) []T {
	if _, ok := g.adj[start]; !ok {
		return nil
	}
	visited := make(map[T]bool)
	var result []T
	g.dfs(start, visited, &result)
	return result
}

func (g *AdjacencyListGraph[T]) dfs(vertex T, visited map[T]bool, result *[]T) {
	visited[vertex] = true
	*result = append(*result, vertex)
	for _, neighbor := range g.adj[vertex] {
		if !visited[neighbor] {
			g.dfs(neighbor, visited, result)
		}
	}
}

func contains[T comparable](slice []T, value T) bool {
	for _, v := range slice {
		if v == value {
			return true
		}
	}
	return false
}

func removeFromSlice[T comparable](slice []T, value T) []T {
	result := make([]T, 0, len(slice))
	for _, v := range slice {
		if v != value {
			result = append(result, v)
		}
	}
	return result
}

// AdjacencyMatrixGraph is a graph represented as an adjacency matrix.
type AdjacencyMatrixGraph[T comparable] struct {
	vertices []T
	index    map[T]int
	matrix   [][]int
	directed bool
}

// NewAdjacencyMatrixGraph creates a new AdjacencyMatrixGraph.
func NewAdjacencyMatrixGraph[T comparable](directed bool) *AdjacencyMatrixGraph[T] {
	return &AdjacencyMatrixGraph[T]{
		index:    make(map[T]int),
		directed: directed,
	}
}

// VertexCount returns the number of vertices.
func (g *AdjacencyMatrixGraph[T]) VertexCount() int { return len(g.vertices) }

// IsEmpty returns true if the graph is empty.
func (g *AdjacencyMatrixGraph[T]) IsEmpty() bool { return len(g.vertices) == 0 }

// AddVertex adds a vertex.
func (g *AdjacencyMatrixGraph[T]) AddVertex(vertex T) {
	if _, ok := g.index[vertex]; ok {
		return
	}
	g.index[vertex] = len(g.vertices)
	g.vertices = append(g.vertices, vertex)
	for i := range g.matrix {
		g.matrix[i] = append(g.matrix[i], 0)
	}
	g.matrix = append(g.matrix, make([]int, len(g.vertices)))
}

// RemoveVertex removes a vertex.
func (g *AdjacencyMatrixGraph[T]) RemoveVertex(vertex T) error {
	index, ok := g.index[vertex]
	if !ok {
		return ErrNotFound
	}
	g.vertices = append(g.vertices[:index], g.vertices[index+1:]...)
	delete(g.index, vertex)
	g.matrix = append(g.matrix[:index], g.matrix[index+1:]...)
	for i := range g.matrix {
		g.matrix[i] = append(g.matrix[i][:index], g.matrix[i][index+1:]...)
	}
	for i, v := range g.vertices {
		g.index[v] = i
	}
	return nil
}

// AddEdge adds an edge with weight.
func (g *AdjacencyMatrixGraph[T]) AddEdge(u, v T, weight int) {
	g.AddVertex(u)
	g.AddVertex(v)
	g.matrix[g.index[u]][g.index[v]] = weight
	if !g.directed {
		g.matrix[g.index[v]][g.index[u]] = weight
	}
}

// RemoveEdge removes an edge.
func (g *AdjacencyMatrixGraph[T]) RemoveEdge(u, v T) {
	if _, ok := g.index[u]; !ok {
		return
	}
	if _, ok := g.index[v]; !ok {
		return
	}
	g.matrix[g.index[u]][g.index[v]] = 0
	if !g.directed {
		g.matrix[g.index[v]][g.index[u]] = 0
	}
}

// HasEdge returns true if there is an edge from u to v.
func (g *AdjacencyMatrixGraph[T]) HasEdge(u, v T) bool {
	ui, ok := g.index[u]
	if !ok {
		return false
	}
	vi, ok := g.index[v]
	if !ok {
		return false
	}
	return g.matrix[ui][vi] != 0
}

// Weight returns the weight of an edge.
func (g *AdjacencyMatrixGraph[T]) Weight(u, v T) (int, error) {
	ui, ok := g.index[u]
	if !ok {
		return 0, ErrNotFound
	}
	vi, ok := g.index[v]
	if !ok {
		return 0, ErrNotFound
	}
	return g.matrix[ui][vi], nil
}

// Neighbors returns the neighbors of a vertex.
func (g *AdjacencyMatrixGraph[T]) Neighbors(vertex T) []T {
	index, ok := g.index[vertex]
	if !ok {
		return nil
	}
	var result []T
	for i, w := range g.matrix[index] {
		if w != 0 {
			result = append(result, g.vertices[i])
		}
	}
	return result
}

// BFS returns vertices in breadth-first order.
func (g *AdjacencyMatrixGraph[T]) BFS(start T) []T {
	if _, ok := g.index[start]; !ok {
		return nil
	}
	visited := make(map[T]bool)
	visited[start] = true
	queue := []T{start}
	var result []T
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		result = append(result, current)
		for _, neighbor := range g.Neighbors(current) {
			if !visited[neighbor] {
				visited[neighbor] = true
				queue = append(queue, neighbor)
			}
		}
	}
	return result
}

// DFS returns vertices in depth-first order.
func (g *AdjacencyMatrixGraph[T]) DFS(start T) []T {
	if _, ok := g.index[start]; !ok {
		return nil
	}
	visited := make(map[T]bool)
	var result []T
	g.dfs(start, visited, &result)
	return result
}

func (g *AdjacencyMatrixGraph[T]) dfs(vertex T, visited map[T]bool, result *[]T) {
	visited[vertex] = true
	*result = append(*result, vertex)
	for _, neighbor := range g.Neighbors(vertex) {
		if !visited[neighbor] {
			g.dfs(neighbor, visited, result)
		}
	}
}
