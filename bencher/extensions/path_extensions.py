from __future__ import annotations

import os
from pathlib import Path
from typing import Union, Iterable

# Didn't find how to retrieve this string in-runtime
MODULE_NAME = 'extensions.'


# TODO: extension-ы уровня БОГ


def get_root_directory() -> Path:
    # TODO: что-то очень странное - что оно делает?
    # Module directory
    file_path = Path(__file__).absolute().parent

    for i in range(MODULE_NAME.count('.')):
        file_path = file_path.parent

    return file_path


class PathDoesNotExist(RuntimeError):
    def __init__(self, path: Union[str, Path]) -> None:
        super().__init__(f'Path {path} does not exist')


def path(path: str | Path):
    p = Path(path).absolute()
    if not p.exists():
        raise PathDoesNotExist(path)
    return p


def path_listdir(path: str | Path) -> Iterable[Path]:
    path = Path(path)
    for p in os.listdir(path):
        yield path / p
