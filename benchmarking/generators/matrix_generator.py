import json
from pathlib import Path
import numpy as np

GENERATORS_CONFIG: Path = Path("generators/generators-config.json")
MATRIX_A_PATH: Path = Path("data/matrix_a.csv")
MATRIX_B_PATH: Path = Path("data/matrix_b.csv")


def generate_matrix(filepath: Path, size: int, max_value: int) -> None:
    matrix = np.random.randint(max_value, size=(size, size))
    np.savetxt(filepath, matrix, header=str(size), delimiter=',', fmt="%d")


def generate_matrices() -> None:
    with open(GENERATORS_CONFIG, "rt") as gen_config_file:
        gen_config_data: str = gen_config_file.read()

    gen_config: any = json.loads(gen_config_data)

    matrix_size: int = gen_config["matrix"]["size"]
    matrix_max_value: int = gen_config["matrix"]["max_value"]
    generate_matrix(MATRIX_A_PATH, matrix_size, matrix_max_value)
    generate_matrix(MATRIX_B_PATH, matrix_size, matrix_max_value)
