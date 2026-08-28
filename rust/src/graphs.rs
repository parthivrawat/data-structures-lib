use std::collections::HashMap;

use crate::Error;

/// A graph represented as an adjacency list.
#[derive(Debug, Clone)]
pub struct AdjacencyListGraph<T: Clone + Eq + std::hash::Hash> {
    adj: HashMap<T, Vec<T>>,
    directed: bool,
}

impl<T: Clone + Eq + std::hash::Hash> AdjacencyListGraph<T> {
    /// Creates a new AdjacencyListGraph.
    pub fn new(directed: bool) -> Self {
        Self {
            adj: HashMap::new(),
            directed,
        }
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.adj.len()
    }

    /// Returns true if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.adj.is_empty()
    }

    /// Adds a vertex.
    pub fn add_vertex(&mut self, vertex: T) {
        self.adj.entry(vertex).or_insert_with(Vec::new);
    }

    /// Removes a vertex.
    pub fn remove_vertex(&mut self, vertex: &T) -> Result<(), Error> {
        if !self.adj.contains_key(vertex) {
            return Err(Error::NotFound);
        }
        for neighbors in self.adj.values_mut() {
            if let Some(pos) = neighbors.iter().position(|v| v == vertex) {
                neighbors.remove(pos);
            }
        }
        self.adj.remove(vertex);
        Ok(())
    }

    /// Adds an edge.
    pub fn add_edge(&mut self, u: T, v: T) {
        self.add_vertex(u.clone());
        self.add_vertex(v.clone());
        if !self.adj[&u].contains(&v) {
            self.adj.get_mut(&u).unwrap().push(v.clone());
        }
        if !self.directed && !self.adj[&v].contains(&u) {
            self.adj.get_mut(&v).unwrap().push(u);
        }
    }

    /// Removes an edge.
    pub fn remove_edge(&mut self, u: &T, v: &T) {
        if let Some(neighbors) = self.adj.get_mut(u) {
            if let Some(pos) = neighbors.iter().position(|x| x == v) {
                neighbors.remove(pos);
            }
        }
        if !self.directed {
            if let Some(neighbors) = self.adj.get_mut(v) {
                if let Some(pos) = neighbors.iter().position(|x| x == u) {
                    neighbors.remove(pos);
                }
            }
        }
    }

    /// Returns true if there is an edge from u to v.
    pub fn has_edge(&self, u: &T, v: &T) -> bool {
        self.adj
            .get(u)
            .map(|neighbors| neighbors.contains(v))
            .unwrap_or(false)
    }

    /// Returns the neighbors of a vertex.
    pub fn neighbors(&self, vertex: &T) -> Vec<T> {
        self.adj.get(vertex).cloned().unwrap_or_default()
    }

    /// Returns vertices in breadth-first order.
    pub fn bfs(&self, start: &T) -> Vec<T> {
        if !self.adj.contains_key(start) {
            return Vec::new();
        }
        let mut visited = std::collections::HashSet::new();
        visited.insert(start.clone());
        let mut queue = vec![start.clone()];
        let mut result = Vec::new();
        while let Some(current) = queue.pop() {
            result.push(current.clone());
            for neighbor in &self.adj[&current] {
                if visited.insert(neighbor.clone()) {
                    queue.insert(0, neighbor.clone());
                }
            }
        }
        result
    }

    /// Returns vertices in depth-first order.
    pub fn dfs(&self, start: &T) -> Vec<T> {
        if !self.adj.contains_key(start) {
            return Vec::new();
        }
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        self.dfs_visit(start, &mut visited, &mut result);
        result
    }

    fn dfs_visit(&self, vertex: &T, visited: &mut std::collections::HashSet<T>, result: &mut Vec<T>) {
        if !visited.insert(vertex.clone()) {
            return;
        }
        result.push(vertex.clone());
        for neighbor in &self.adj[vertex] {
            self.dfs_visit(neighbor, visited, result);
        }
    }
}

/// A graph represented as an adjacency matrix.
#[derive(Debug, Clone)]
pub struct AdjacencyMatrixGraph<T: Clone + Eq + std::hash::Hash> {
    vertices: Vec<T>,
    index: HashMap<T, usize>,
    matrix: Vec<Vec<i32>>,
    directed: bool,
}

impl<T: Clone + Eq + std::hash::Hash> AdjacencyMatrixGraph<T> {
    /// Creates a new AdjacencyMatrixGraph.
    pub fn new(directed: bool) -> Self {
        Self {
            vertices: Vec::new(),
            index: HashMap::new(),
            matrix: Vec::new(),
            directed,
        }
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns true if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Adds a vertex.
    pub fn add_vertex(&mut self, vertex: T) {
        if self.index.contains_key(&vertex) {
            return;
        }
        self.index.insert(vertex.clone(), self.vertices.len());
        self.vertices.push(vertex);
        for row in &mut self.matrix {
            row.push(0);
        }
        self.matrix.push(vec![0; self.vertices.len()]);
    }

    /// Removes a vertex.
    pub fn remove_vertex(&mut self, vertex: &T) -> Result<(), Error> {
        let idx = *self.index.get(vertex).ok_or(Error::NotFound)?;
        self.vertices.remove(idx);
        self.index.remove(vertex);
        self.matrix.remove(idx);
        for row in &mut self.matrix {
            row.remove(idx);
        }
        for (i, v) in self.vertices.iter().enumerate() {
            self.index.insert(v.clone(), i);
        }
        Ok(())
    }

    /// Adds an edge with weight.
    pub fn add_edge(&mut self, u: T, v: T, weight: i32) {
        self.add_vertex(u.clone());
        self.add_vertex(v.clone());
        let ui = self.index[&u];
        let vi = self.index[&v];
        self.matrix[ui][vi] = weight;
        if !self.directed {
            self.matrix[vi][ui] = weight;
        }
    }

    /// Removes an edge.
    pub fn remove_edge(&mut self, u: &T, v: &T) {
        if let (Some(&ui), Some(&vi)) = (self.index.get(u), self.index.get(v)) {
            self.matrix[ui][vi] = 0;
            if !self.directed {
                self.matrix[vi][ui] = 0;
            }
        }
    }

    /// Returns true if there is an edge from u to v.
    pub fn has_edge(&self, u: &T, v: &T) -> bool {
        if let (Some(&ui), Some(&vi)) = (self.index.get(u), self.index.get(v)) {
            return self.matrix[ui][vi] != 0;
        }
        false
    }

    /// Returns the weight of an edge.
    pub fn weight(&self, u: &T, v: &T) -> Result<i32, Error> {
        let ui = *self.index.get(u).ok_or(Error::NotFound)?;
        let vi = *self.index.get(v).ok_or(Error::NotFound)?;
        Ok(self.matrix[ui][vi])
    }

    /// Returns the neighbors of a vertex.
    pub fn neighbors(&self, vertex: &T) -> Vec<T> {
        let idx = match self.index.get(vertex) {
            Some(&i) => i,
            None => return Vec::new(),
        };
        let mut result = Vec::new();
        for (i, w) in self.matrix[idx].iter().enumerate() {
            if *w != 0 {
                result.push(self.vertices[i].clone());
            }
        }
        result
    }

    /// Returns vertices in breadth-first order.
    pub fn bfs(&self, start: &T) -> Vec<T> {
        if !self.index.contains_key(start) {
            return Vec::new();
        }
        let mut visited = std::collections::HashSet::new();
        visited.insert(start.clone());
        let mut queue = vec![start.clone()];
        let mut result = Vec::new();
        while let Some(current) = queue.pop() {
            result.push(current.clone());
            for neighbor in self.neighbors(&current) {
                if visited.insert(neighbor.clone()) {
                    queue.insert(0, neighbor);
                }
            }
        }
        result
    }

    /// Returns vertices in depth-first order.
    pub fn dfs(&self, start: &T) -> Vec<T> {
        if !self.index.contains_key(start) {
            return Vec::new();
        }
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        self.dfs_visit(start, &mut visited, &mut result);
        result
    }

    fn dfs_visit(&self, vertex: &T, visited: &mut std::collections::HashSet<T>, result: &mut Vec<T>) {
        if !visited.insert(vertex.clone()) {
            return;
        }
        result.push(vertex.clone());
        for neighbor in self.neighbors(vertex) {
            self.dfs_visit(&neighbor, visited, result);
        }
    }
}
