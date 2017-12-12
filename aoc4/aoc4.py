count = 0
all = 0

# part 1
with open("input") as f:
	for line in f:
		all += 1
		line = line.strip().split()
		if len(line) == len(set(line)):
			count += 1
print(count, all)

count2 = 0
# part 2

import operator
from functools import reduce 
with open("input") as f:
	for line in f:
		words = map(hash, line.strip().split())
		res = reduce(operator.xor, words)
		#print(res)
		if res == 0:
			print(line)
			count2 += 1


print(count2)

			
		