#!/usr/bin/env sage

num_terms = 10
prime = 0x40000000000000000000000000000000224698fc094cf91b992d30ed00000001
n = len(format(prime, 'b'))
F = GF(prime)

# The print_hex and print_words_to_hex functions are from https://github.com/daira/pasta-hadeshash
# (with pallas::Base::from_raw replaced with Fp::from_raw)
def print_hex(c, last, rust=False):
    c = int(c)
    if rust:
        print("        Fp::from_raw([")
        for i in range(0, n, 64):
            print("            0x%04x_%04x_%04x_%04x," % tuple([(c >> j) & 0xFFFF for j in range(i+48, i-1, -16)]))
        print("        ]),")
    else:
        hex_length = (n + 3)//4 + 2 # +2 for "0x"
        print("{0:#0{1}x}".format(c, hex_length), end="" if last else ", ")

def print_words_to_hex(M, rust=False):
    print("    [", end="\n" if rust else "")
    for (i, entry) in enumerate(M):
        print_hex(entry, i == len(M)-1, rust=rust)
    print("    ]," if rust else "],")


def main(args):
    rust = '--rust' in args

    fib_squares = [F(1), F(1)]

    for i in range(num_terms-2):
        next_term = fib_squares[-2]^2 + fib_squares[-1]^2
        fib_squares.append(next_term)

    print_words_to_hex(fib_squares, rust=rust)

if __name__ == "__main__":
    import sys
    main(sys.argv[1:])