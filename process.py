#!/usr/bin/python

import sys

if len(sys.argv) != 2:
    print("exiting")
    sys.exit()

# Format is `tag file start stop`, where `start` and `stop` are line numbers
RUST_TAG = "rcode"
JAVA_TAG = "jcode"

name = sys.argv[1]
print("File =", name)

fin = open(name, 'r')
fout = open(name+".out", 'w')

for line in fin:
    tokens = line.replace('\n','').split(' ')
    print(tokens)
    if tokens[0] == RUST_TAG or tokens[0] == JAVA_TAG:
        loc = tokens[1]
        f = open(loc, 'r')
        if len(tokens) == 4:
            start = int(tokens[2])
            stop = int(tokens[3])
        else:
            start = 0
            stop = 1000 # arbitrarily chose some large number
        i = 1
        for l in f:
            if start <= i and i <= stop:
                print("writing", l, end='')
                fout.write(l)
            i += 1
        f.close()
    else:
        fout.write(line)

fin.close()
fout.close()
