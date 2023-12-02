import subprocess
from dataclasses import dataclass
from pathlib import Path

from extensions.logging_extensions import LOGGER, Color
from extensions.path_extensions import get_root_directory, path_must_exist
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
    result: TestResult = None

    @classmethod
    def from_config(cls, data: dict[str, any]) -> "Test":
        test_path: Path = Path(get_root_directory() / data["path"])
        path_must_exist(Path(test_path))
        return Test(
            path=test_path,
            commands=[Command.from_string(it) for it in data["commands"]],
        )

    def exec_test(self, pipeline_name: str, language_name: str) -> None:
        results = [it.exec() for it in self.commands]
        if pipeline_name == "Versions":
            LOGGER.info(f"Language -> {language_name}")
            LOGGER.info(f"Version:\n"
                        f"{results[-1].decode('ascii')}")
        else:
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
                LOGGER.debug(f"{language_name} -> {test_name}")
                self.languages[language_name].tests[test_name].exec_test(pipeline.name, language_name)
                if pipeline.name != "Versions":
                    LOGGER.info(self.get_test_result(language_name, test_name))

    def get_test_result(self, language_name: str, test_name: str) -> str:

        result = self.languages[language_name].tests[test_name].result.get_format_result()
        return f"Language -> {language_name}\n" \
               f"Test -> {test_name}\n" \
               f"{result}"

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
        return self.languages[lang].tests[test].result.real_time
