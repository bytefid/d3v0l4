"""Round game arithmetic to Rust's IEEE 754 f32 after each operation.

Timers and simulation speed intentionally remain ordinary Python f64 floats.
"""

import ctypes
import ctypes.util
import math
import sys


class F32(float):
    def __new__(cls, value=0.0):
        return super().__new__(cls, ctypes.c_float(value).value)

    def __add__(self, other):
        return F32(float(self) + float(other))

    __radd__ = __add__

    def __sub__(self, other):
        return F32(float(self) - float(other))

    def __rsub__(self, other):
        return F32(float(other) - float(self))

    def __mul__(self, other):
        return F32(float(self) * float(other))

    __rmul__ = __mul__

    def __truediv__(self, other):
        return F32(float(self) / float(other))

    def __rtruediv__(self, other):
        return F32(float(other) / float(self))

    def __neg__(self):
        return F32(-float(self))

    def __abs__(self):
        return F32(abs(float(self)))

    def rem_euclid(self, divisor):
        remainder = F32(math.fmod(self, divisor))
        return remainder + abs(divisor) if remainder < 0.0 else remainder

    def to_radians(self):
        return self * F32(math.pi / 180.0)


def clamp(value, low, high):
    return min(max(value, low), high)


# Rust uses the platform's single-precision trigonometric functions. Calling
# these avoids rounding a double-precision sin/cos differently by one f32 ULP.
_math_lib = ctypes.CDLL("ucrtbase.dll" if sys.platform == "win32" else ctypes.util.find_library("m") or None)
_sinf = _math_lib.sinf
_cosf = _math_lib.cosf
for _function in (_sinf, _cosf):
    _function.argtypes = [ctypes.c_float]
    _function.restype = ctypes.c_float


def sin(value):
    return F32(_sinf(value))


def cos(value):
    return F32(_cosf(value))
