import sys

ls = sys.stdin.readlines()
for l in ls:
    [_, name, value] = l.split()
    ns = name.split('_')[2:]
    nl = ''.join(n[0] + n[1:].lower() for n in ns)
    print("%s = %s, " % (nl, value))
