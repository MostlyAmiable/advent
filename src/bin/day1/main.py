from pathlib import Path

INPUT_PATH = Path(__file__).parent.joinpath("input.txt")


def main():
    part_one()
    part_two()


def part_one():
    print(load_input()[0])


def part_two():
    print(sum(load_input()[0:3]))


def load_input() -> list[int]:
    sections = open(INPUT_PATH).read().split("\n\n")
    calories = map(lambda section: sum(map(int, section.splitlines())), sections)
    return sorted(calories, reverse=True)


if __name__ == '__main__':
    main()
