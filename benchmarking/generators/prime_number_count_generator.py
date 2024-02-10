import json
from pathlib import Path
import random

PRIME_NUMBER_COUNT_PATH: Path = Path("data/prime_number_count.txt")
GENERATORS_CONFIG: Path = Path("generators/generators-config.json")


def generate_prime_number_count() -> None:
    with open(GENERATORS_CONFIG, "rt") as gen_config_file:
        gen_config_data: str = gen_config_file.read()

    gen_config: any = json.loads(gen_config_data)

    prime_number_count_min_value: int = gen_config["prime_number_count"]["min_value"]
    prime_number_count_max_value: int = gen_config["prime_number_count"]["max_value"]

    prime_number_count: int = random.randint(prime_number_count_min_value, prime_number_count_max_value)

    with open(PRIME_NUMBER_COUNT_PATH, "wt") as prime_number_count_file:
        prime_number_count_file.write(str(prime_number_count))
        prime_number_count_file.write("\n")
