


class Node:
    def __init__(self, is_terminal: bool = False) -> None:
        self.is_terminal = is_terminal
        self.next = {}


class Trie:
    def __init__(self) -> None:
        self.root = Node()
        self.state = []
        self.words = []

    def add(self, word: str) -> None:
        current_node = self.root
        for char in word:
            if char not in current_node.next:
                current_node.next[char] = Node()
            current_node = current_node.next[char]
        current_node.is_terminal = True

    def dfs(self, node: Node | None = None) -> None:
        if node is None:
            node = self.root

        for char in sorted(node.next.keys()):
            self.state.append(char)
            if node.next[char].is_terminal:
                word = ''.join(self.state)
                self.words.append(word)
            self.dfs(node.next[char])
            self.state.pop()


if __name__ == "__main__":
    trie = Trie()
    trie.add("apple")
    trie.add("app")
    trie.add("application")
    trie.add("banana")
    trie.add("band")
    trie.add("cherry")
    trie.dfs()
    print(f"State after DFS: {trie.state}")
    print(f"All words found: {trie.words}")






a = 1
b = 2
c = a + b
