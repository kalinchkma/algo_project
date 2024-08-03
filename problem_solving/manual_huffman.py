import heapq


class HuffmanNode:

    char = None
    freq = None

    def __init__(self, char, freq) -> None:
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None

    # comparision operator
    def __lt__(self, other):
        return self.freq < other.freq

    def __str__(self) -> str:
        return f"{self.char} {self.freq}"


def create_huffman_tree(chars):
    freq = {}

    # freqs
    for c in chars:
        if c in freq:
            freq[c] += 1
        else:
            freq[c] = 1

    # Create tree
    heap = [HuffmanNode(c, f) for c, f in freq.items()]

    heapq.heapify(heap)

    huffman_tree = HuffmanNode(None, None)

    print(len(heap))
    heapq.heappop(heap)
    print(len(heap))


if __name__ == "__main__":
    s = "AABACDACA"
    huffman_tree = create_huffman_tree(s)
