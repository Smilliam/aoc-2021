def part1(filename):
    horizontal_pos = 0
    depth = 0
    with open(filename, 'r') as ff:
        for line in ff:
            split = line.split()
            direction = split[0]
            distance = int(split[1])

            if direction == 'forward':
                horizontal_pos += distance
            elif direction == 'down':
                depth += distance
            elif direction == 'up':
                depth -= distance

    product = horizontal_pos * depth

    print("part 1: " + str(product))

def part2(filename):
    horizontal_pos = 0
    depth = 0
    aim = 0
    with open(filename, 'r') as ff:
        for line in ff:
            split = line.split()
            direction = split[0]
            distance = int(split[1])

            if direction == 'forward':
                horizontal_pos += distance
                depth += aim * distance
            elif direction == 'down':
                aim += distance
            elif direction == 'up':
                aim -= distance

    product = horizontal_pos * depth

    print("part 2: " + str(product))


def parse_args():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('in_file', type=str)
    return parser.parse_args()

def main():
    args = parse_args()

    in_file = args.in_file

    part1(in_file)
    part2(in_file)


if __name__ == '__main__':
    main()
