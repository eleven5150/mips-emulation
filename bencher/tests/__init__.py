from dataclasses import dataclass
from pathlib import Path
from typing import Dict
from typing import Generator

from extensions.json_extensions import get_parsed_config_generic, DataclassDaciteStrictMixin
from extensions.path_extensions import path_must_exist, get_root_directory

TESTS_CONFIG_ROOT_DIR = Path(__file__).resolve().parent
SCHEMA_JSON_FILEPATH = TESTS_CONFIG_ROOT_DIR / 'tests-config.schema.json'


@dataclass
class TestsConfigData(DataclassDaciteStrictMixin):
    name: str
    description: str
    languages: Dict[str, Dict[str, Dict[str, str]]]

    def __iter__(self) -> Generator[str, None, None]:
        for key, value in self.languages:
            yield value


def __get_tests_config_path() -> Path:
    return TESTS_CONFIG_ROOT_DIR / "tests-config.json"


def get_tests_config_data() -> TestsConfigData:
    tests_config_path: Path = __get_tests_config_path()
    path_must_exist(tests_config_path)
    return get_parsed_config_generic(
        cls=TestsConfigData,
        data_path=tests_config_path,
        schema_path=SCHEMA_JSON_FILEPATH
    )