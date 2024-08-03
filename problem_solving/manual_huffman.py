class HuffmanNode:

    char = None
    freq = None

    def __init__(self, char, freq) -> None:
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None

    def __str__(self) -> str:
        return f"{self.char} {self.freq}"


def create_huffman_tree():
    n = HuffmanNode("A", 2)
    print(n)


if __name__ == "__main__":
    s = "AABACDACA"
    huffman_tree = create_huffman_tree()
