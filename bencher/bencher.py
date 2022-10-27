import argparse
import logging
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
# TODO: не надо юзать Dict, List, Set и т.п. из typing
#  3.9.10+ держит dict[str, Any], если версия ниже нужно поднять
from typing import Dict, Any, Optional

from extensions.path_extensions import path, get_root_directory
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

    def exec(self) -> TestResult:
        logging.debug(f"\tCommand -> {' '.join(self.cmd)}")
        process = subprocess.Popen(self.cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        out, err = process.communicate()
        assert process.returncode == 0, f"'{self.cmd} execution failed with status {process.returncode}"
        logging.debug(f"\tReturn -> {out}")
        return TestResult.from_stdout(out)


@dataclass
class Test:
    path: Path
    commands: list[Command]
    # TODO: Зачем эти состояния хранить в классе? пусть возвращаются из exec
    result: Optional[TestResult] = None

    @classmethod
    def from_config(cls, data: dict[str, Any]) -> "Test":
        # TODO: вообще эти проверки на путь вероятнее всего не нужны, они у тебя походу все
        #  равно не обрабатываются, а Path сам чекнет, что пути нету, короче so-so
        return Test(
            path=path(Path(get_root_directory()) / data["path"]),
            commands=[Command.from_string(it) for it in data["commands"]],
        )

    def exec_test(self) -> None:
        # TODO: не очень понятно как предполагается извлекать результаты, если несколько команд?
        #  по последней команде? Тогда лучше так и прописать в код, что все выполняем без разбора
        #  результата, а ластовую разбираем. Тот костыль с объявлением в начале - ужасен
        #  И да парсить тогда их надо по разному, вероятно нужно сделать несколько типов команд
        #  наследованных от одного протокола и также несколько типов результатов
        results = [it.exec() for it in self.commands]
        return TestResult.from_command_result(results)


    # TODO: Метод больше не нужен, оставил из-за других комментариев
    def convert_commands(cls, commands: list[str]) -> None:
        # TODO: не надо просто так писать тип переменной при объявлении, тем более без указания внутреннего типа
        #  это только сбивает PyCharm - у него есть шанс :) вывести внутренний тип
        #  Попробуй убрать тип и посмотри, что выведет PyCharm list[list[str]]
        #  Если ты напишешь просто list - то вся инфа о внутреннем типе будет потеряна
        #  Такие аннотации только сбивают, либо надо писать тип полностью, либо не писать
        converted_cmds = list()

        # TODO: вот эта ваша любовь к глобальным переменным и запихиваний состояний куда угодно
        #  нужно переделать и убрать из состояния класса
        #  Да из команды можно класс сделать, а не
        self.commands = converted_cmds



# TODO: Дальше не смотрел
@dataclass
class ProgLang:
    tests: Dict[str, Test]

    @classmethod
    def data_to_prog_lang(cls, language_data: dict[str, dict[str, str]]) -> "ProgLang":
        tests: Dict[str, Test] = dict()
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
                logging.debug(f"{Colors.WARNING}{language_name} -> {test_name}{Colors.ENDC}")
                self.languages[language_name].tests[test_name].exec_test()
                logging.info(self.get_test_result(language_name, test_name))

    def get_test_result(self, language_name: str, test_name: str) -> str:
        # TODO: Выглядит ужасно
        real_time = self.languages[language_name].tests[test_name].result.real_time
        user_time = self.languages[language_name].tests[test_name].result.user_time
        sys_time = self.languages[language_name].tests[test_name].result.sys_time
        # https://plugins.jetbrains.com/plugin/7125-grep-console
        return f"{Colors.OKGREEN}Language -> {language_name}{Colors.ENDC}\n" \
               f"{Colors.OKGREEN}Test -> {test_name}{Colors.ENDC}\n" \
               f"{Colors.OKGREEN}Real time -> {real_time} s{Colors.ENDC}\n" \
               f"{Colors.OKGREEN}User-mode time -> {user_time} s{Colors.ENDC}\n" \
               f"{Colors.OKGREEN}Kernel-mode time -> {sys_time} s{Colors.ENDC}\n"

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
