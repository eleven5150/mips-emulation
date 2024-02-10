from dataclasses import dataclass
import re
from pathlib import Path

from extensions.json_extensions import DataclassDaciteStrictMixin, get_parsed_config_generic
from extensions.path_extensions import path_listdir, path_must_exist

PIPELINE_CONFIG_ROOT_DIR = Path(__file__).resolve().parent
SCHEMA_JSON_FILEPATH = PIPELINE_CONFIG_ROOT_DIR / 'pipeline.schema.json'
if not SCHEMA_JSON_FILEPATH.exists():
    raise RuntimeError(f"File {SCHEMA_JSON_FILEPATH} does not exist")


NOT_TEST_PIPELINES: list[str] = [
    "full",
    "versions",
    "local"
]


class NotTestPipelineException(Exception):
    pass


@dataclass
class Pipeline(DataclassDaciteStrictMixin):
    name: str
    description: str
    test: str
    pipeline: dict[str, list]

    def print_pipeline(self) -> None:
        print("Pipeline:")
        for language_name in self.pipeline:
            print(f"\t Language - {language_name}:")
            for test in self.pipeline[language_name]:
                print(f"\t\t{test}")
        print("\n")

    def get_languages_names_for_test(self, test: str) -> tuple[str]:
        languages_names: list[str] = list()
        for language_name in self.pipeline:
            if test in self.pipeline[language_name]:
                languages_names.append(language_name)
        return tuple(languages_names)

    def get_unique_languages(self) -> set[str]:
        unique_languages: set[str] = set()
        for language_name in self.pipeline:
            unique_languages.add(language_name)
        return unique_languages


def __get_pipeline_path(name: str) -> Path:
    return PIPELINE_CONFIG_ROOT_DIR / f'pipeline-{name}.json'


def get_all_pipeline_names() -> list[str]:
    file_wildcard = re.compile(r"pipeline-([^.]+)\.json")
    return [
        m.group(1)
        for m in (
            re.match(file_wildcard, p.name)
            for p in path_listdir(PIPELINE_CONFIG_ROOT_DIR)
        )
        if m is not None
    ]


def get_pipeline(name: str) -> Pipeline:
    path = __get_pipeline_path(name)
    path_must_exist(path)

    return get_parsed_config_generic(
        cls=Pipeline,
        data_path=path,
        schema_path=SCHEMA_JSON_FILEPATH
    )
