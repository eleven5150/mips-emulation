import subprocess
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
class TestResult:
    version: str
    real_time: int
    user_time: int
    sys_time: int

    def __init__(self):
        self.version = "unknown"
        self.real_time = 0
        self.user_time = 0
        self.sys_time = 0

    def parse_stdout(self, result_data: bytes):
        print(result_data)


@dataclass
class Test:
    path: Path
    cmd: list
    result: TestResult

    @classmethod
    def data_to_test(cls, data: Dict[str, str]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        return Test(
            path=test_path,
            cmd=data["cmd"].split(" "),
            result=TestResult()
        )

    def exec_test(self):
        result_data = subprocess.run(
            self.cmd,
            capture_output=True
        )
        self.result.parse_stdout(result_data.stdout)

@dataclass
class ProgLang:
    tests: Dict[str, Test]

    @classmethod
    def data_to_prog_lang(cls, language_data: Dict[str, Dict[str, str]]) -> "ProgLang":
        tests: Dict[str, Test] = dict()
        for test_name in language_data:
            tests.update({test_name: Test.data_to_test(language_data[test_name])})
        return ProgLang(
            tests=tests
        )


@dataclass
class TestsConfig:
    name: str
    description: str
    languages: Dict[str, ProgLang]

    def exec_pipeline(self, pipeline: Pipeline):
        for language_name in pipeline.pipeline:
            for test_name in pipeline.pipeline[language_name]:
                self.languages[language_name].tests[test_name].exec_test()

    @classmethod
    def data_to_tests_config(cls, data: TestsConfigData) -> "TestsConfig":
        target_languages: Dict[str, ProgLang] = dict()
        for language_name in data.languages:
            target_languages.update({language_name: ProgLang.data_to_prog_lang(data.languages[language_name])})
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
    tests_config.exec_pipeline(pipeline)
    # for language_name in pipeline.pipeline:
    #     for test_name in pipeline.pipeline[language_name]:
    #         print(f"{language_name} -> {test_name}")



if __name__ == '__main__':
    main(sys.argv)
