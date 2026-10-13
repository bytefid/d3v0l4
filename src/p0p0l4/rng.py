"""Python equivalent of rand 0.10.3 StdRng (PCG-seeded ChaCha12).

The algorithm and float sampling follow rand_core 0.10.1, chacha20 0.10.2
and rand 0.10.3 so the original seed produces the same game.
"""

import math

from .numeric import F32

_MASK32 = (1 << 32) - 1
_MASK64 = (1 << 64) - 1


def _rotate_left(value, count):
    return ((value << count) | (value >> (32 - count))) & _MASK32


def _quarter_round(words, a, b, c, d):
    words[a] = (words[a] + words[b]) & _MASK32
    words[d] = _rotate_left(words[d] ^ words[a], 16)
    words[c] = (words[c] + words[d]) & _MASK32
    words[b] = _rotate_left(words[b] ^ words[c], 12)
    words[a] = (words[a] + words[b]) & _MASK32
    words[d] = _rotate_left(words[d] ^ words[a], 8)
    words[c] = (words[c] + words[d]) & _MASK32
    words[b] = _rotate_left(words[b] ^ words[c], 7)


class StdRng:
    def __init__(self, seed):
        if not 0 <= seed <= _MASK64:
            raise ValueError("seed must fit in an unsigned 64-bit integer")
        key = []
        for _ in range(8):
            seed = (seed * 0x5851F42D4C957F2D + 0xA17654E46FBE17F3) & _MASK64
            value = (((seed >> 18) ^ seed) >> 27) & _MASK32
            rotation = seed >> 59
            key.append(((value >> rotation) | (value << ((-rotation) & 31))) & _MASK32)
        self._state = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574, *key, 0, 0, 0, 0]
        self._buffer = []
        self._index = 0

    def next_u32(self):
        if self._index == len(self._buffer):
            words = self._state.copy()
            for _ in range(6):
                for indices in ((0, 4, 8, 12), (1, 5, 9, 13), (2, 6, 10, 14), (3, 7, 11, 15),
                                (0, 5, 10, 15), (1, 6, 11, 12), (2, 7, 8, 13), (3, 4, 9, 14)):
                    _quarter_round(words, *indices)
            self._buffer = [(a + b) & _MASK32 for a, b in zip(words, self._state)]
            self._index = 0
            self._state[12] = (self._state[12] + 1) & _MASK32
            if self._state[12] == 0:
                self._state[13] = (self._state[13] + 1) & _MASK32
        value = self._buffer[self._index]
        self._index += 1
        return value

    def next_u64(self):
        low = self.next_u32()
        return low | (self.next_u32() << 32)

    def random_range(self, low, high):
        low, high = F32(low), F32(high)
        if not (low <= high and math.isfinite(low) and math.isfinite(high - low)):
            raise ValueError("invalid f32 range")
        unit = F32((self.next_u32() >> 9) / (1 << 23))
        return unit * (high - low) + low

    def random_bool(self, probability):
        if not 0.0 <= probability <= 1.0:
            raise ValueError("probability must be between zero and one")
        if probability == 1.0:
            return True
        return self.next_u64() < int(probability * (1 << 64))


def new_rng(seed):
    return StdRng(seed)
