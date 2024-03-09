import os
import subprocess
from dataclasses import dataclass
from pathlib import Path

from extensions.logging_extensions import LOGGER, Color
from extensions.path_extensions import get_root_directory, path_must_exist, path_safe_mkdir, path_listdir
from generator import Generator
from pipeline import Pipeline
from tests.settings import DOCKER_COMMAND
from tests.tests_config_data import TestsConfigData


@dataclass
class TestResult:
    real_time: float = 0.0
    user_time: float = 0.0
    sys_time: float = 0.0

    @classmethod
    def from_stdout(cls, data: bytes) -> "TestResult":
        if not data:
            raise ValueError(Color.error("Error! Test execution returned empty data"))
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
        cmd_split_raw: list[str] = DOCKER_COMMAND.split(" ")
        cmd_split_raw.extend(cmd.split(" "))
        cmd_converted: list[str] = cls.convert_arguments_in_single_quotes(cmd_split_raw)
        return cls(cmd_converted)

    def exec(self) -> bytes:
        LOGGER.debug(f"\tCommand -> {' '.join(self.cmd)}")
        process = subprocess.Popen(self.cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        out, err = process.communicate()
        # TODO fix this
        # assert process.returncode == 0, Color.error(
        #     f"'{' '.join(self.cmd)} execution failed with status {process.returncode}\n\n"
        #     f"Error message: {str(out, encoding='ascii')}"
        # )
        LOGGER.debug(f"\tReturn -> \n{str(out, encoding='ascii')}")
        return out

    @staticmethod
    def convert_arguments_in_single_quotes(cmd_split_raw: list[str]) -> list[str]:
        cmd_converted: list[str] = list()
        for arg in cmd_split_raw:
            if "'" in arg:
                start_quote: int = cmd_split_raw.index(arg)
                merged_argument: str = " ".join(cmd_split_raw[start_quote:])
                converted_argument: str = merged_argument[1:-1]
                cmd_converted.append(converted_argument)
                break
            cmd_converted.append(arg)
        return cmd_converted


@dataclass
class Test:
    path: Path
    commands: list[Command]
    result: list[TestResult]

    @classmethod
    def from_config(cls, data: dict[str, any]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        return cls(
            path=test_path,
            commands=[Command.from_string(it) for it in data["commands"]],
            result=list()
        )

    @classmethod
    def from_results(cls, results_file_path: Path) -> "Test":
        result: list[TestResult] = list()
        with open(results_file_path, "rt") as results_file:
            results_data: str = results_file.read()

        results_lines: list[str] = results_data.split("\n")
        results_lines.pop(-1)
        for line in results_lines:
            time_str: str = line.split(":")[1]
            if time_str == "None":
                time_str = "0"
            time: float = float(time_str)
            result.append(TestResult(real_time=time))
        return cls(
            path=results_file_path,
            commands=list(),
            result=result
        )

    def exec_test(self, pipeline_name: str, language_name: str) -> None:
        results = [it.exec() for it in self.commands]
        if pipeline_name == "Versions":
            LOGGER.info(f"Language -> {language_name}")
            LOGGER.info(f"Version:\n"
                        f"{results[-1].decode('ascii')}")
        else:
            self.result.append(TestResult.from_stdout(results[-1]))


@dataclass
class ProgLang:
    tests: dict[str, Test]

    @classmethod
    def data_to_prog_lang(cls, language_data: dict[str, dict[str, str]]) -> "ProgLang":
        tests: dict[str, Test] = dict()
        for test_name in language_data:
            tests.update({test_name: Test.from_config(language_data[test_name])})
        return ProgLang(tests=tests)

    @classmethod
    def results_to_prog_lang(cls, lang_dir_path: Path) -> "ProgLang":
        tests: dict[str, Test] = dict()
        for test_dir_path in path_listdir(lang_dir_path):
            if os.path.isdir(test_dir_path):
                tests.update({test_dir_path.name: Test.from_results(Path(test_dir_path / "output.txt"))})
        return ProgLang(tests=tests)


@dataclass
class TestsConfig:
    name: str
    description: str
    languages: dict[str, ProgLang]

    def exec_pipeline(self, pipeline: Pipeline, count: int) -> None:
        for it in range(count):
            Generator.generate_all()
            for language_name in pipeline.pipeline:
                for test_name in pipeline.pipeline[language_name]:
                    LOGGER.debug(f"{language_name} -> {test_name}")
                    self.languages[language_name].tests[test_name].exec_test(pipeline.name, language_name)
                    if pipeline.name != "Versions":
                        LOGGER.info(self.get_test_result(language_name, test_name, it))
                        self.save_test_result(language_name, test_name, it)

    def get_test_result(self, language_name: str, test_name: str, number: int) -> str:
        result: TestResult = self.languages[language_name].tests[test_name].result[number]
        result_string: str = result.get_format_result()
        return f"Language -> {language_name}\n" \
               f"Test -> {test_name}\n" \
               f"Number -> {number + 1}\n" \
               f"{result_string}"

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

    def get_result_by_lang_and_test(self, lang: str, test: str) -> float:
        sum_of_times: int = 0
        for result in self.languages[lang].tests[test].result:
            sum_of_times += result.real_time

        return sum_of_times / len(self.languages[lang].tests[test].result)

    def save_test_result(self, language_name: str, test_name: str, number: int) -> None:
        output_dir: Path = Path(get_root_directory() / "output")
        path_safe_mkdir(output_dir)
        language_dir: Path = Path(output_dir / language_name)
        path_safe_mkdir(language_dir)
        test_dir: Path = Path(language_dir / test_name)
        path_safe_mkdir(test_dir)
        output_file_path: Path = Path(test_dir / "output.txt")
        if number == 0 and os.path.exists(output_file_path):
            os.remove(output_file_path)
        result: TestResult = self.languages[language_name].tests[test_name].result[number]
        with open(output_file_path, "a") as output_file:
            output_file.write(f"{number + 1}:{result.real_time}\n")

    @classmethod
    def results_to_test_config(cls, results_dir: Path) -> "TestsConfig":
        target_languages: dict[str, ProgLang] = dict()
        for lang_dir_path in path_listdir(results_dir):
            if os.path.isdir(lang_dir_path):
                target_languages.update({lang_dir_path.name: ProgLang.results_to_prog_lang(lang_dir_path)})
        return cls(
            name="Results",
            description="Test results from logs",
            languages=target_languages
        )
