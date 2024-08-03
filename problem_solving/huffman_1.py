import heapq
from collections import defaultdict


# huffman node to build tree
class HuffmanNode:
    def __init__(self, char, freq):
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None

    def __lt__(self, other):
        return self.freq < other.freq

    def __str__(self) -> str:
        return f"c: {self.char} f: {self.freq}"


def build_frequency_dict(text):
    frequency = defaultdict(int)
    for char in text:
        frequency[char] += 1
    return frequency


def build_huffman_tree(frequency):
    print("Frequency", frequency)
    heap = [HuffmanNode(char, freq) for char, freq in frequency.items()]

    for h in heap:
        print(h)

    heapq.heapify(heap)
    for h in heap:
        print(h)

    while len(heap) > 1:
        right = heapq.heappop(heap)
        left = heapq.heappop(heap)
        print("left", left)
        print("right", right)

        merged = HuffmanNode(None, left.freq + right.freq)
        merged.left = left
        merged.right = right

        print("Merged: c", merged.char, "F", merged.freq)

        heapq.heappush(heap, merged)

    return heap[0]


def build_huffman_codes(node, current_code, huffman_codes):
    if node is None:
        return

    if node.char is not None:
        huffman_codes[node.char] = current_code
        return

    build_huffman_codes(node.left, current_code + "0", huffman_codes)
    build_huffman_codes(node.right, current_code + "1", huffman_codes)


def huffman_encoding(text):
    frequency = build_frequency_dict(text)
    huffman_tree = build_huffman_tree(frequency)
    huffman_codes = {}
    build_huffman_codes(huffman_tree, "", huffman_codes)

    encoded_text = "".join(huffman_codes[char] for char in text)
    return encoded_text, huffman_codes


def huffman_decoding(encoded_text, huffman_codes):
    reversed_codes = {code: char for char, code in huffman_codes.items()}
    decoded_text = ""
    current_code = ""

    for bit in encoded_text:
        current_code += bit
        if current_code in reversed_codes:
            decoded_text += reversed_codes[current_code]
            current_code = ""

    return decoded_text


# Example usage
text = "AABACDACA"
encoded_text, huffman_codes = huffman_encoding(text)
print("Encoded text:", encoded_text)
print("Huffman Codes:", huffman_codes)
print("Decoded text:", huffman_decoding(encoded_text, huffman_codes))
