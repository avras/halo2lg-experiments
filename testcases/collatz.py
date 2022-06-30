# Example from https://medium.com/starkware/arithmetization-i-15c046390862
# A Collatz sequence starts with any positive integer n.
# If n is even, the next element is n/2. Otherwise, it is 3*n+1. 
# The Collatz conjecture says that the sequence always ends in 1
#
# Example with n = 52 
# 52 -> 26 -> 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

from math import floor, log2
import sys

def collatz_sequence(n):
    seq = []
    while n != 1:
        seq.append(n)
        if n % 2 == 0:
            n = int(n/2)
        else:
            n = 3*n+1
    seq.append(1)
    return seq

if len(sys.argv[1:]) > 0:
    n = int(sys.argv[1])
else:
    n = 52
n = max(n, 1)
print('n =', n)
cseq = collatz_sequence(n)
print(cseq)
print('Length of sequence =', len(cseq))
print('Number of bits required to represent largest sequence member =', floor(log2(max(cseq)))+1)
print('Big-endian representation of initial sequence member n =', bin(n)[2:])

