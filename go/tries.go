package datastructures

// trieNode is a node in the Trie.
type trieNode struct {
	children map[rune]*trieNode
	isEnd    bool
	count    int
}

// Trie is a prefix tree for string storage.
type Trie struct {
	root *trieNode
	size int
}

// NewTrie creates a new Trie.
func NewTrie() *Trie {
	return &Trie{
		root: &trieNode{children: make(map[rune]*trieNode)},
	}
}

// Insert adds a word to the trie.
func (t *Trie) Insert(word string) {
	node := t.root
	for _, char := range word {
		if node.children[char] == nil {
			node.children[char] = &trieNode{children: make(map[rune]*trieNode)}
		}
		node = node.children[char]
		node.count++
	}
	if !node.isEnd {
		t.size++
		node.isEnd = true
	}
}

// Search returns true if the word is in the trie.
func (t *Trie) Search(word string) bool {
	node := t.find(word)
	return node != nil && node.isEnd
}

// StartsWith returns true if any word starts with the prefix.
func (t *Trie) StartsWith(prefix string) bool {
	return t.find(prefix) != nil
}

func (t *Trie) find(word string) *trieNode {
	node := t.root
	for _, char := range word {
		if node.children[char] == nil {
			return nil
		}
		node = node.children[char]
	}
	return node
}

// Delete removes a word from the trie.
func (t *Trie) Delete(word string) error {
	if !t.Search(word) {
		return ErrNotFound
	}
	t.delete(t.root, []rune(word), 0)
	t.size--
	return nil
}

func (t *Trie) delete(node *trieNode, word []rune, index int) {
	if index == len(word) {
		node.isEnd = false
		return
	}
	char := word[index]
	child := node.children[char]
	t.delete(child, word, index+1)
	child.count--
	if child.count == 0 {
		delete(node.children, char)
	}
}

// Words returns all words with the given prefix.
func (t *Trie) Words(prefix string) []string {
	var node *trieNode
	if prefix == "" {
		node = t.root
	} else {
		node = t.find(prefix)
	}
	if node == nil {
		return nil
	}
	var result []string
	t.collect(node, []rune(prefix), &result)
	return result
}

func (t *Trie) collect(node *trieNode, prefix []rune, result *[]string) {
	if node.isEnd {
		*result = append(*result, string(prefix))
	}
	for char, child := range node.children {
		t.collect(child, append(prefix, char), result)
	}
}

// Len returns the number of words.
func (t *Trie) Len() int { return t.size }

// IsEmpty returns true if the trie is empty.
func (t *Trie) IsEmpty() bool { return t.size == 0 }
