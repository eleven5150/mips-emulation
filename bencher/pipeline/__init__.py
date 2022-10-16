import dataclasses
import re
from pathlib import Path
from typing import List, Generator

from extensions import json_extensions
from extensions.path_extensions import path_listdir

PIPELINE_CONFIG_ROOT_DIR = Path(__file__).resolve().parent
SCHEMA_JSON_FILEPATH = PIPELINE_CONFIG_ROOT_DIR / 'pipeline.schema.json'
if not SCHEMA_JSON_FILEPATH.exists():
    raise RuntimeError(f"File {SCHEMA_JSON_FILEPATH} does not exist")


@dataclasses.dataclass
class Pipeline(json_extensions.DataclassDaciteStrictMixin):
    name: str
    description: str
    pipeline: List[str]

    def __iter__(self) -> Generator[str, None, None]:
        for name in self.pipeline:
            yield name


class ImageToolPipelineNotFound(RuntimeError):
    def __init__(self, name: str, path: Path):
        super().__init__(f'Pipeline {name} not found. '
                         f'Searched path: {path}')


def __get_pipeline_path(name: str) -> Path:
    return PIPELINE_CONFIG_ROOT_DIR / f'pipeline-{name}.json'


def get_all_pipeline_names() -> List[str]:
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
    if not path.exists():
        raise ImageToolPipelineNotFound(name, path)

    return json_extensions.get_parsed_config_generic(
        cls=Pipeline,
        data_path=path,
        schema_path=SCHEMA_JSON_FILEPATH
    )
