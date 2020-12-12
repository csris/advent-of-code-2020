import argparse
import functools
import operator

def find_vals(vals, n, sum):
    if n == 0:
        return []

    if n == 1:
        return [v for v in vals if v == sum]

    for i, v in enumerate(vals):
        found = find_vals(vals[:i] + vals[i + 1:], n - 1, sum - v)

        if len(found) > 0:
            return [v] + found

    return []

def main():
    parser = argparse.ArgumentParser(description='validates an expense report')
    parser.add_argument('file', metavar='FILE', help='the expense report')
    parser.add_argument('--num', action='store', type=int, default=2, help='number of numbers to find')
    parser.add_argument('--sum', action='store', type=int, default=2020, help='sum of the numbers')

    args = parser.parse_args()

    with open(args.file, 'r') as f:
        vals = [int(l) for l in f]

    found = find_vals(vals, args.num, args.sum)

    check_value = functools.reduce(operator.mul, found)

    print(check_value)

if __name__ == '__main__':
    main()
