import sys
import os
from ctypes import cdll
from ctypes import c_char_p, c_int, c_char

__all__ = ["pi_func", "load_pi","compute"]

__version__ = "3.14.1592"

__author__="tlming16"

pilib = None
pi_func = None


def load_pi():
    global pilib
    global pi_func
    this_file = __file__
    base_path = os.path.dirname(this_file)
    so_files = ["lib/lib_pi.so", "lib/lib_pi"]
    if pi_func is not None:
        return pi_func
    if pilib is None:
        for so in so_files:
            cp_path = os.path.join(base_path, so);
            try:
                pilib = cdll.LoadLibrary(cp_path)
                break
            except:
                pilib = None
                continue
    if pilib is None:
        print("lib_pi.so doesn't exist")
        exit(1)
    if pi_func is None:
        pi_func = pilib.pi
        pi_func.argtypes = [c_int, c_char_p]
        pi_func.restype = c_char_p
    return pi_func


pi_func = load_pi()

def compute(default_digit=4000):
    global pi_func
    if pi_func is None:
        pi_func = load_pi()
    if pi_func is None:
        print("pi is not installed")
        exit(1)
    buf_type = c_char * (default_digit + 3)
    ans = buf_type()
    ans = pi_func(default_digit, ans)
    ans = ans.decode('utf-8')
    return ans
