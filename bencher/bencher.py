import argparse
import logging
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from extensions.path_extensions import path_must_exist, get_root_directory
from pipeline import Pipeline, get_pipeline, get_all_pipeline_names
from tests import TestsConfigData, get_tests_config_data

TESTS_CONFIG: str = "tests/tests-config.json"


# TODO: Все раскрасски нужно убрать либо в методы, либо переделать на grep-console
#  точнее нужно обязательно ее поставить и уже в сообщении выводить инфу для расскрашивания
#  И да совершенно непонятно, почему лог сообщения уровня debug содержит warning? Это как?
# https://plugins.jetbrains.com/plugin/7125-grep-console


@dataclass
class TestResult:
    real_time: float = 0.0
    user_time: float = 0.0
    sys_time: float = 0.0

    @classmethod
    def from_stdout(cls, data: bytes) -> "TestResult":
        if not data:
            raise ValueError(f"Error! Test execution returned empty data")
        real_time = user_time = sys_time = None
        for line in data.decode("ascii").splitlines():
            if "real" in line:
                real_time = float(line.split(" ")[1])
            if "user" in line:
                user_time = float(line.split(" ")[1])
            if "sys" in line:
                sys_time = float(line.split(" ")[1])
        return cls(real_time, user_time, sys_time)

    def get_format_result(self) -> str:
        return f"Real time -> {self.real_time} s\n" \
               f"User-mode time -> {self.user_time} s\n" \
               f"Kernel-mode time -> {self.sys_time} s\n"


@dataclass
class Command:
    cmd: list[str]

    @classmethod
    def from_string(cls, cmd: str) -> "Command":
        cmd_split_raw = cmd.format(get_root_directory()).split(" ")
        for arg in cmd_split_raw:
            if "'" in arg:
                # TODO: однозначно нужно передалать - тут просто не понятно что происходит
                cmd_split_raw[cmd_split_raw.index(arg):len(cmd_split_raw)] = [
                    " ".join(cmd_split_raw[cmd_split_raw.index(arg):len(cmd_split_raw)])]
                cmd_split_raw[len(cmd_split_raw) - 1] = cmd_split_raw[len(cmd_split_raw) - 1][:-1]
                cmd_split_raw[len(cmd_split_raw) - 1] = cmd_split_raw[len(cmd_split_raw) - 1][1:]
        return cls(cmd_split_raw)

    def exec(self) -> bytes:
        logging.debug(f"\tCommand -> {' '.join(self.cmd)}")
        process = subprocess.Popen(self.cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        out, err = process.communicate()
        assert process.returncode == 0, f"'{self.cmd} execution failed with status {process.returncode}"
        logging.debug(f"\tReturn -> {out}")
        return out


@dataclass
class Test:
    path: Path
    commands: list[Command]
    result: Optional[TestResult] = None

    @classmethod
    def from_config(cls, data: dict[str, any]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        return Test(
            path=test_path,
            commands=[Command.from_string(it) for it in data["commands"]],
        )

    def exec_test(self) -> None:
        # TODO: не очень понятно как предполагается извлекать результаты, если несколько команд?
        #  по последней команде? Тогда лучше так и прописать в код, что все выполняем без разбора
        #  результата, а ластовую разбираем. Тот костыль с объявлением в начале - ужасен
        #  И да парсить тогда их надо по разному, вероятно нужно сделать несколько типов команд
        #  наследованных от одного протокола и также несколько типов результатов
        results = [it.exec() for it in self.commands]
        self.result = TestResult.from_stdout(results[-1])


@dataclass
class ProgLang:
    tests: dict[str, Test]

    @classmethod
    def data_to_prog_lang(cls, language_data: dict[str, dict[str, str]]) -> "ProgLang":
        tests: dict[str, Test] = dict()
        for test_name in language_data:
            tests.update({test_name: Test.from_config(language_data[test_name])})
        return ProgLang(tests=tests)


@dataclass
class TestsConfig:
    name: str
    description: str
    languages: dict[str, ProgLang]

    def exec_pipeline(self, pipeline: Pipeline):
        for language_name in pipeline.pipeline:
            for test_name in pipeline.pipeline[language_name]:
                logging.debug(f"{language_name} -> {test_name}")
                self.languages[language_name].tests[test_name].exec_test()
                logging.info(self.get_test_result(language_name, test_name))

    def get_test_result(self, language_name: str, test_name: str) -> str:
        result = self.languages[language_name].tests[test_name].result.get_format_result()
        return f"Language -> {language_name}\n" \
               f"Test -> {test_name}\n" \
               f"{result}" \


    @classmethod
    def data_to_tests_config(cls, data: TestsConfigData) -> "TestsConfig":
        target_languages: dict[str, ProgLang] = dict()
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
                        help='Creates an image graph.jpeg with a graph comparing execution speeds')
    parser.add_argument('-d', '--debug',
                        action='store_true',
                        help='Enables debug mode')

    return parser.parse_args(arguments)


def main(raw_arguments: list) -> None:
    args = parse_args(raw_arguments[1:])

    if args.debug:
        logging.basicConfig(format='%(message)s', level=logging.DEBUG)
    else:
        logging.basicConfig(format='%(message)s', level=logging.INFO)

    pipeline: Pipeline = get_pipeline(args.pipeline)
    logging.info(f"Pipeline name -> {pipeline.name}")
    logging.info(f"Pipeline description -> {pipeline.description}")
    pipeline.print_pipeline()
    tests_config_data: TestsConfigData = get_tests_config_data()
    tests_config: TestsConfig = TestsConfig.data_to_tests_config(tests_config_data)
    tests_config.exec_pipeline(pipeline)


if __name__ == '__main__':
    main(sys.argv)
