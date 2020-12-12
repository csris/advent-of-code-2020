import argparse
import re

def part1_validate(min, max, ch, password):
    count = len([c for c in password if c == ch])
    return min <= count and count <= max

def part2_validate(i1, i2, ch, password):
    return (
        (password[i1 - 1] != password[i2 - 1]) 
        and (password[i1 - 1] == ch or password[i2 - 1] == ch)
    )

def count_valid(passfile, validation_fn):
    regex = re.compile('([0-9]+)-([0-9]+) ([a-z]): (.*)')

    valid_count = 0

    with open(passfile, 'r') as f:
        for line in f:
            match = regex.match(line.strip())
            (a, b, ch, password) = match.groups()

            a = int(a)
            b = int(b)

            if validation_fn(a, b, ch, password):
                valid_count += 1

    return valid_count

def main():
    parser = argparse.ArgumentParser(description='validate password file')
    parser.add_argument('passfile', metavar='PASSWORD_FILE', help='the password file')
    parser.add_argument('--part2', action='store_const', const=part2_validate, default=part1_validate, dest='validator', help='use the part2 validation function (default: use part1)')

    args = parser.parse_args()

    valid_count = count_valid(args.passfile, args.validator);

    print(valid_count)

if __name__ == '__main__':
    main()
