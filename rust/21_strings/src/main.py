x = "hello"
y = "hello"
print(f"1: x is y = {x is y}")
print(f"  id(x) = {id(x)}")
print(f"  id(y) = {id(y)}")


x = "hell"
x = x + "o"
print(f"2: x is y = {x is y}")
print(f"  id(x) = {id(x)}")
print(f"  id(y) = {id(y)}")


import sys
x = sys.intern(x)
print(f"3: x is y = {x is y}")
print(f"  id(x) = {id(x)}")
print(f"  id(y) = {id(y)}")

x = "hell" + "o"
print(f"4: x is y? - {x is y}")
print(f"  id(x) = {id(x)}")
print(f"  id(y) = {id(y)}")