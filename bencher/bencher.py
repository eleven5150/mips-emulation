import subprocess
import sys
import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Any

from extensions.path_extensions import path_must_exist, get_root_directory
from pipeline import Pipeline, get_pipeline, get_all_pipeline_names
from tests import TestsConfigData, get_tests_config_data

TESTS_CONFIG: str = "tests/tests-config.json"


@dataclass
class TestResult:
    real_time: float
    user_time: float
    sys_time: float

    def __init__(self):
        self.real_time = 0
        self.user_time = 0
        self.sys_time = 0

    def parse_stdout(self, result_data: bytes) -> None:
        if result_data == b"":
            raise ValueError("Error! Test execution returned empty data")
        datas: list = str(result_data, encoding="ascii").split("\n")
        for line in datas:
            if "real" in line:
                self.real_time = line.split(" ")[1]
            if "user" in line:
                self.user_time = line.split(" ")[1]
            if "sys" in line:
                self.sys_time = line.split(" ")[1]


@dataclass
class Test:
    path: Path
    commands: list
    result: TestResult

    @classmethod
    def data_to_test(cls, data: Dict[str, Any]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        commands_raw: list = data["commands"]
        return Test(
            path=test_path,
            commands=commands_raw,
            result=TestResult()
        )

    def exec_test(self) -> None:
        self.convert_commands()
        result_data: bytes = bytes()
        for cmd in self.commands:
            # print(f"Command -> '{cmd}'")
            result_data = subprocess.check_output(
                cmd,
                stderr=subprocess.STDOUT
            )

        self.result.parse_stdout(result_data)

    def convert_commands(self) -> None:
        converted_cmds: list = list()
        for cmd in self.commands:
            cmd_split_raw: list = cmd.format(get_root_directory()).split(" ")
            for arg in cmd_split_raw:
                if "'" in arg:
                    cmd_split_raw[cmd_split_raw.index(arg):len(cmd_split_raw)] = [
                        " ".join(cmd_split_raw[cmd_split_raw.index(arg):len(cmd_split_raw)])]
                    cmd_split_raw[len(cmd_split_raw) - 1] = cmd_split_raw[len(cmd_split_raw) - 1][:-1]
                    cmd_split_raw[len(cmd_split_raw) - 1] = cmd_split_raw[len(cmd_split_raw) - 1][1:]
            converted_cmds.append(cmd_split_raw)
        self.commands = converted_cmds


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
                print(self.get_test_result(language_name, test_name))

    def get_test_result(self, language_name: str, test_name: str) -> str:
        return f"Language -> {language_name}\n" \
               f"Test -> {test_name}\n" \
               f"Real time -> {self.languages[language_name].tests[test_name].result.real_time} s\n" \
               f"User-mode time -> {self.languages[language_name].tests[test_name].result.user_time} s\n" \
               f"Kernel-mode time -> {self.languages[language_name].tests[test_name].result.sys_time} s\n"

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
                        choices=get_all_pipeline_names(),
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


if __name__ == '__main__':
    main(sys.argv)
