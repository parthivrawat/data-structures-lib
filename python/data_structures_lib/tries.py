'''Trie data structure.'''

from typing import Dict, Iterator, List, Optional

__all__ = ['Trie']


class _TrieNode:
    __slots__ = ('children', 'is_end', 'count')

    def __init__(self) -> None:
        self.children: Dict[str, '_TrieNode'] = {}
        self.is_end = False
        self.count = 0


class Trie:
    '''A prefix tree for string storage.'''

    def __init__(self, words: Optional[List[str]] = None) -> None:
        self._root = _TrieNode()
        self._size = 0
        if words is not None:
            for word in words:
                self.insert(word)

    def __len__(self) -> int:
        return self._size

    def is_empty(self) -> bool:
        return self._size == 0

    def insert(self, word: str) -> None:
        '''Insert a word into the trie.'''
        if not isinstance(word, str):
            raise TypeError('Trie only supports str keys')
        node = self._root
        for char in word:
            if char not in node.children:
                node.children[char] = _TrieNode()
            node = node.children[char]
            node.count += 1
        if not node.is_end:
            self._size += 1
            node.is_end = True

    def _find(self, word: str) -> Optional[_TrieNode]:
        node = self._root
        for char in word:
            if char not in node.children:
                return None
            node = node.children[char]
        return node

    def search(self, word: str) -> bool:
        '''Return True if word is in the trie.'''
        if not isinstance(word, str):
            raise TypeError('Trie only supports str keys')
        node = self._find(word)
        return node is not None and node.is_end

    def starts_with(self, prefix: str) -> bool:
        '''Return True if any word starts with prefix.'''
        if not isinstance(prefix, str):
            raise TypeError('Trie only supports str keys')
        return self._find(prefix) is not None

    def delete(self, word: str) -> None:
        '''Delete a word from the trie.'''
        if not self.search(word):
            raise KeyError(word)
        self._delete(self._root, word, 0)
        self._size -= 1

    def _delete(self, node: _TrieNode, word: str, index: int) -> None:
        if index == len(word):
            node.is_end = False
            return
        char = word[index]
        child = node.children[char]
        self._delete(child, word, index + 1)
        child.count -= 1
        if child.count == 0:
            del node.children[char]

    def words(self, prefix: str = '') -> List[str]:
        '''Return all words with the given prefix.'''
        node = self._find(prefix) if prefix else self._root
        if node is None:
            return []
        result: List[str] = []
        self._collect(node, list(prefix), result)
        return result

    def _collect(self, node: _TrieNode, prefix: List[str], result: List[str]) -> None:
        if node.is_end:
            result.append(''.join(prefix))
        for char, child in node.children.items():
            prefix.append(char)
            self._collect(child, prefix, result)
            prefix.pop()

    def __iter__(self) -> Iterator[str]:
        return iter(self.words())

    def __contains__(self, word: str) -> bool:
        return self.search(word)

    def __repr__(self) -> str:
        return f'Trie({self.words()})'
