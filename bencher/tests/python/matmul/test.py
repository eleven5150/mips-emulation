import sys

MATRIX_DIMENSION: int = 0


def matrix_print(matrix: list[list[int]]) -> None:
    for line in matrix:
        for item in line:
            print(f"{item}", end="\t")
        print("")


def create_matrix(file_path: str) -> list[list[int]]:
    global MATRIX_DIMENSION

    with open(file_path, "r") as file:
        data: str = file.read()

    lines: list[str] = data.split("\n")
    MATRIX_DIMENSION = int(lines[0][2:])
    lines.pop(0)
    lines.pop(len(lines) - 1)

    matrix: list[list[int]] = list(list())

    for my_line in lines:
        records: list[str] = my_line.split(",")
        matrix.append([int(it) for it in records])

    return matrix


def matrix_multiply(matrix_dimension: int, matrix_a: list[list[int]], matrix_b: list[list[int]]) -> list[list[int]]:
    t_matrix_b: list[list[int]] = [[0 for it in range(matrix_dimension)] for jit in range(matrix_dimension)]
    for it in range(matrix_dimension):
        for jit in range(matrix_dimension):
            item: int = matrix_b[jit][it]
            t_matrix_b[it][jit] = item

    matrix_result: list[list[int]] = [[0 for it in range(matrix_dimension)] for jit in range(matrix_dimension)]
    for it in range(matrix_dimension):
        for jit in range(matrix_dimension):
            summ: int = 0
            for kit in range(matrix_dimension):
                summ += matrix_a[it][kit] * t_matrix_b[jit][kit]
            matrix_result[it][jit] = summ

    return matrix_result


def main(args: list[str]) -> None:
    if len(args) < 2:
        print("Error! Both paths to matrix data required")
        sys.exit(1)

    matrix_a_file: str = args[1]
    matrix_b_file: str = args[2]

    matrix_a: list[list[int]] = create_matrix(matrix_a_file)
    matrix_b: list[list[int]] = create_matrix(matrix_b_file)

    matrix_result: list[list[int]] = matrix_multiply(MATRIX_DIMENSION, matrix_a, matrix_b)

    matrix_print(matrix_result)


if __name__ == '__main__':
    main(sys.argv)
