import heapq
from collections import defaultdict


# Huffman Node defination
class Node:
    def __init__(self, char, freq):
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None

    # Defining comparison operators for priority queue
    def __lt__(self, other):
        return self.freq < other.freq

    def __eq__(self, other):
        return self.freq == other.freq


# Huffman tree
def build_huffman_tree(frequencies):
    heap = [Node(char, freq) for char, freq in frequencies.items()]
    heapq.heapify(heap)

    print("Huffman...................")
    for h in heap:
        print("heap node:", h.char, h.freq)

    while len(heap) > 1:
        left = heapq.heappop(heap)
        right = heapq.heappop(heap)
        merged = Node(None, left.freq + right.freq)
        merged.left = left
        merged.right = right
        heapq.heappush(heap, merged)
    return heap[0]


# Huffman codes
def build_codes(node, prefix="", codebook={}):
    if node is None:
        return

    if node.char is not None:
        codebook[node.char] = prefix
    else:
        build_codes(node.left, prefix + "0", codebook)
        build_codes(node.right, prefix + "1", codebook)

    return codebook


# Encode
def encode(data, codebook):
    return "".join(codebook[char] for char in data)


# Decode
def decode(encoded_data, root):
    decoded = []
    node = root
    for bit in encoded_data:
        node = node.left if bit == "0" else node.right
        if node.char is not None:
            decoded.append(node.char)
            node = root

    return "".join(decoded)


# main
def main():
    data = "AABACDACA"
    frequencies = defaultdict(int)

    print("frequencies:", frequencies)

    for char in data:
        frequencies[char] += 1

    huffman_tree = build_huffman_tree(frequencies)
    codebook = build_codes(huffman_tree)

    encoded_data = encode(data, codebook)
    decoded_data = decode(encoded_data, huffman_tree)

    print("Original data:", data)
    print("Encoded data:", encoded_data)
    print("Decoded data:", decoded_data)


if __name__ == "__main__":
    main()
