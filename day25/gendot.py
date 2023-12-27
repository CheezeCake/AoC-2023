#!/usr/bin/env python3

import sys

print('graph day25 {')
for line in sys.stdin:
  node, adj_list = line.split(': ')
  for adj in adj_list.split(' '):
    print(f"\t{node} -- {adj}")
print('}')
