import json
from pathlib import Path

import numpy as np

SORT_DATA_PATH: Path = Path("data/sort_data.txt")
GENERATORS_CONFIG: Path = Path("generators/generators-config.json")


def generate_data_to_sort() -> None:
    with open(GENERATORS_CONFIG, "rt") as gen_config_file:
        gen_config_data: str = gen_config_file.read()

    gen_config: any = json.loads(gen_config_data)

    data_to_sort_size: int = gen_config["data_to_sort"]["size"]
    data_to_sort_max_value: int = gen_config["data_to_sort"]["max_value"]

    matrix = np.random.randint(data_to_sort_max_value, size=data_to_sort_size)
    np.savetxt(SORT_DATA_PATH, matrix, header=str(data_to_sort_size), fmt="%d")
