from sys import getsizeof as sizeof
from time import process_time as time
# from time import perf_counter as time
    
num = 10000000
l = []
size = sizeof(l)
oldsize = 0
print(len(l), size)
for i in range(num):
    t1 = time()
    l.append(9999)
    t2 = time()
    size = sizeof(l)
    if size != oldsize: print(len(l), size - 64, f"{t2 - t1:.2e}")
    oldsize = size
