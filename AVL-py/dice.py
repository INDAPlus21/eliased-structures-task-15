import math
import random 

thirtteen = 0
rolls = 100000
sums = []
for i in range(100000):
    rand_1 =  random.randint(1, 7) 
    rand_2 =  random.randint(1, 7) 
    rand_3 =  random.randint(1, 7) 
    sum_roll = rand_1 + rand_2 + rand_3
    sums.append(sum_roll)
    # print(sum_roll)
    if sum_roll == 13:
        thirtteen += 1

print(thirtteen)
print(thirtteen/rolls)
print()

def most_frequent(List):
    return max(set(List), key = List.count)
 
print(most_frequent(sums))
