from pathlib import Path
import numpy as np

MATRIX_SIZE: int = 100


def generate_matrix(filepath: Path) -> None:
    matrix = np.random.randint(0xFFFFFFFF, size=(MATRIX_SIZE, MATRIX_SIZE))
    np.savetxt(filepath, matrix, delimiter=',', fmt="%d")


def generate_matrices() -> None:
    generate_matrix(Path("./data/matrix_a.csv"))
    generate_matrix(Path("./data/matrix_b.csv"))
