import sys

DATA_TO_SORT_SIZE: int = 0


def data_print(data: list[int]) -> None:
    for item in data:
        print(item)


def partition(data_to_sort: list[int], low: int, high: int) -> int:
    pivot: int = data_to_sort[high]
    it: int = low - 1

    for jit in range(low, high):
        if data_to_sort[jit] <= pivot:
            it += 1
            data_to_sort[it], data_to_sort[jit] = data_to_sort[jit], data_to_sort[it]

    data_to_sort[it + 1], data_to_sort[high] = data_to_sort[high], data_to_sort[it + 1]

    return it + 1


def quick_sort(data_to_sort: list[int], low: int, high: int) -> list[int]:
    if low < high:
        pi: int = partition(data_to_sort, low, high)

        quick_sort(data_to_sort, low, pi - 1)
        quick_sort(data_to_sort, pi + 1, high)

    return data_to_sort


def main(args: list[str]) -> None:
    global DATA_TO_SORT_SIZE

    if len(args) < 1:
        print("Error! File with data to sort must be specified")
        sys.exit(1)

    data_to_sort_path: str = args[1]
    with open(data_to_sort_path, "r") as file:
        data: str = file.read()

    lines: list[str] = data.split("\n")
    lines.pop(len(lines) - 1)

    DATA_TO_SORT_SIZE = int(lines[0][2:])
    lines.pop(0)

    data_to_sort: list[int] = [int(it) for it in lines]

    sorted_data: list[int] = quick_sort(data_to_sort, 0, DATA_TO_SORT_SIZE - 1)

    data_print(sorted_data)


if __name__ == '__main__':
    main(sys.argv)
    