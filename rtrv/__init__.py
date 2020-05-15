from rtrv._native import ffi

import sys, ctypes
from ctypes import c_void_p
import pprint

lib = ctypes.cdll.LoadLibrary("rtrv/rtrv.dll")

lib.get.restype = c_void_p

def get():

    ptr = lib.get()
    return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')

def getListDict():
    return eval(get())

def returnDesiredOutputWithTests():
    print("Rust: Get products from rust code...\n")

    data = get()

    if type(data) == str and len(data):
        print("Nice! Data not empty and returned as str.\n")
    else:
        print("Fail! Data empty or not of expected type.\n")
        return -1

    print("Converting data to Python DIctionary\n")

    result = eval(data)

    print("Nice! Data converted successfully!\n")

    if (len(result) == 3):
        print("Nice! Python dictionary has all 3 products\n")
    else:
        print("Fail! Not all data were stored.\n")
        return -1

    for r in result:
        if (type(r) != dict):
            print("Fail! Not all data are dictionary.\n")
            return -1

    print("Nice! All data are dictionary.\n")

    if (len(list(result[0].keys())) == 4):
        print("Nice! Complete Field..\n")
    else:
        print("Fail! Missing Field.\n")


    pp = pprint.PrettyPrinter(indent=4, depth=2)
    pp.pprint(result)

    print("\nTests ran successfully!\n")
