import json
import random
from dataclasses import dataclass
from pathlib import Path

import numpy as np

from generator.settings import GENERATORS_CONFIG


@dataclass
class GeneratorConfig:
    config_json: any

    @classmethod
    def config_data_to_test_generator(cls, config_path: Path) -> "GeneratorConfig":
        with open(config_path, "rt") as gen_config_file:
            gen_config_data: str = gen_config_file.read()

        gen_config: any = json.loads(gen_config_data)
        return cls(
            config_json=gen_config
        )


@dataclass
class QuickSortGenerator:
    QUICK_SORT_DATA_PATH: Path = Path("data/data_to_sort.txt")

    @classmethod
    def generate(cls, gen_conf: GeneratorConfig) -> None:
        data_to_sort_size: int = gen_conf.config_json["data_to_sort"]["size"]
        data_to_sort_max_value: int = gen_conf.config_json["data_to_sort"]["max_value"]

        matrix = np.random.randint(data_to_sort_max_value, size=data_to_sort_size)
        np.savetxt(cls.QUICK_SORT_DATA_PATH, matrix, header=str(data_to_sort_size), fmt="%d")


@dataclass
class MatMulGenerator:
    MATMUL_A_DATA_PATH: Path = Path("data/matrix_a.csv")
    MATMUL_B_DATA_PATH: Path = Path("data/matrix_b.csv")

    @classmethod
    def generate(cls, gen_conf: GeneratorConfig) -> None:
        matrix_size: int = gen_conf.config_json["matrix"]["size"]
        matrix_max_value: int = gen_conf.config_json["matrix"]["max_value"]
        cls.generate_matrix(cls.MATMUL_A_DATA_PATH, matrix_size, matrix_max_value)
        cls.generate_matrix(cls.MATMUL_B_DATA_PATH, matrix_size, matrix_max_value)

    @staticmethod
    def generate_matrix(filepath: Path, size: int, max_value: int) -> None:
        matrix = np.random.randint(max_value, size=(size, size))
        np.savetxt(filepath, matrix, header=str(size), delimiter=',', fmt="%d")


@dataclass
class PrimeNumberGenerator:
    PRIME_NUMBER_DATA_PATH: Path = Path("data/prime_number_count.txt")

    @classmethod
    def generate(cls, gen_conf: GeneratorConfig) -> None:
        prime_number_count_min_value: int = gen_conf.config_json["prime_number_count"]["min_value"]
        prime_number_count_max_value: int = gen_conf.config_json["prime_number_count"]["max_value"]

        prime_number_count: int = random.randint(prime_number_count_min_value, prime_number_count_max_value)

        with open(cls.PRIME_NUMBER_DATA_PATH, "wt") as prime_number_count_file:
            prime_number_count_file.write(str(prime_number_count))
            prime_number_count_file.write("\n")


@dataclass
class Generator:
    quick_sort: QuickSortGenerator
    matmul: MatMulGenerator
    prime_number: PrimeNumberGenerator

    @classmethod
    def generate_all(cls, gen_conf: GeneratorConfig) -> None:
        cls.quick_sort.generate(gen_conf)
        cls.matmul.generate(gen_conf)
        cls.prime_number.generate(gen_conf)

    @classmethod
    def generate_quick_sort(cls, gen_conf: GeneratorConfig) -> None:
        cls.quick_sort.generate(gen_conf)

    @classmethod
    def generate_matmul(cls, gen_conf: GeneratorConfig) -> None:
        cls.matmul.generate(gen_conf)

    @classmethod
    def generate_prime_number(cls, gen_conf: GeneratorConfig) -> None:
        cls.prime_number.generate(gen_conf)
