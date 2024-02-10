import sys


def main(args: list[str]) -> None:
    if len(args) < 1:
        print("Error! File with prime number count must be specified")
        sys.exit(1)

    prime_number_count_file: str = args[1]

    with open(prime_number_count_file, "r") as file:
        prime_number_count: int = int(file.read())

    number: int = 0
    while prime_number_count > 0:
        number += 1
        j: int = 0

        for i in range(1, number + 1):
            if number % i == 0:
                j += 1

        if j == 2:
            prime_number_count -= 1

    print(f"The latest prime number: {number}")


if __name__ == "__main__":
    main(sys.argv)
