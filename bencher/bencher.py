import os
import sys
import json
import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import List, Union

from extensions.json_extensions import get_parsed_config_generic
from extensions.path_extensions import path_must_exist
from pipeline import Pipeline, get_pipeline
from tests import TestsConfig, get_tests_config

TESTS_CONFIG: str = "tests/tests-config.json"


@dataclass
class Test:
    name: str
    path: Path
    cmd: str

    @classmethod
    def data_to_test(cls, name: str, data: dict) -> "Test":
        return Test(
            name=name,
            path=Path(data["path"]),
            cmd=data["cmd"]
        )


@dataclass
class ProgLang:
    name: str
    tests: List[Test]

    @classmethod
    def data_to_prog_lang(cls, name: str, data: dict) -> "ProgLang":
        tests: List[Test] = list()
        for test in data:
            tests.append(Test.data_to_test(test, data[test]))
        return ProgLang(
            name=name,
            tests=tests
        )


def get_languages_and_tests() -> List[ProgLang]:

    with open(tests_config_path) as tests_config:
        tests_config_data = json.load(tests_config)
    prog_langs: List[ProgLang] = list()
    for lang in tests_config_data:
        prog_langs.append(ProgLang.data_to_prog_lang(lang, tests_config_data[lang]))
    return prog_langs


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
    tests_config: TestsConfig = get_tests_config()
    print(tests_config)


if __name__ == '__main__':
    main(sys.argv)
