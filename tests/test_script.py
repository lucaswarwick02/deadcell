# test_script.py

unused_var = 42
unused_imported_var = 99

def used_function():
    used_local = 10  # used local variable
    unused_local = 20  # unused local variable
    print(used_local)

def unused_function():
    pass

class UsedClass:
    def method(self):
        used_function()
        unused_method_local = 5  # unused local inside method

class UsedOnlyInNotebook:
    def hello(self):
        print("Hello from notebook!")

class UnusedClass:
    pass

import math  # unused import
import json as js  # imported alias, unused
import os as operating_system  # alias import that will be used

used_function()
obj = UsedClass()
obj.method()
