import sys
import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import List, Dict

from extensions.path_extensions import path_must_exist, get_root_directory
from pipeline import Pipeline, get_pipeline
from tests import TestsConfigData, get_tests_config_data


TESTS_CONFIG: str = "tests/tests-config.json"


@dataclass
class Test:
    name: str
    path: Path
    cmd: list

    @classmethod
    def data_to_test(cls, test_name: str, data: Dict[str, str]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        return Test(
            name=test_name,
            path=test_path,
            cmd=data["cmd"].split(" ")
        )


@dataclass
class ProgLang:
    name: str
    tests: List[Test]

    @classmethod
    def data_to_prog_lang(cls, language_name: str, language_data: Dict[str, Dict[str, str]]) -> "ProgLang":
        tests: List[Test] = list()
        for test_name in language_data:
            tests.append(Test.data_to_test(test_name, language_data[test_name]))
        return ProgLang(
            name=language_name,
            tests=tests
        )


@dataclass
class TestsConfig:
    name: str
    description: str
    languages: List[ProgLang]

    @classmethod
    def data_to_tests_config(cls, data: TestsConfigData) -> "TestsConfig":
        target_languages: List[ProgLang] = list()
        for language_name in data.languages:
            target_languages.append(ProgLang.data_to_prog_lang(language_name, data.languages[language_name]))
        return TestsConfig(
            name=data.name,
            description=data.description,
            languages=target_languages
        )


def parse_args(arguments: list):
    parser = argparse.ArgumentParser(description="Bench tool or comparing the speed of programming languages")
    parser.add_argument('-p', '--pipeline',
                        type=str,
                        required=True,
                        help='Pipeline for testing')
    parser.add_argument('-o', '--output_file',
                        type=str,
                        help='Path to file with test result')
    parser.add_argument('-i', '--image',
                        action='store_true',
                        help='Creates an image graph.jpeg with a graph comparing execution speeds. ')

    return parser.parse_args(arguments)


def main(raw_arguments: list) -> None:
    args = parse_args(raw_arguments[1:])
    pipeline: Pipeline = get_pipeline(args.pipeline)
    tests_config_data: TestsConfigData = get_tests_config_data()
    tests_config: TestsConfig = TestsConfig.data_to_tests_config(tests_config_data)
    print(tests_config)


if __name__ == '__main__':
    main(sys.argv)
