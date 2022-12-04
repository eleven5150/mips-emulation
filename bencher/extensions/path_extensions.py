from __future__ import annotations

import os
from pathlib import Path
from typing import Iterable

# Didn't find how to retrieve this string in-runtime
MODULE_NAME = 'extensions.'


def get_root_directory() -> Path:
    file_path = Path(__file__).absolute().parent

    for i in range(MODULE_NAME.count('.')):
        file_path = file_path.parent

    return file_path


class PathDoesNotExist(RuntimeError):
    def __init__(self, my_path: str | Path) -> None:
        super().__init__(f'Path {my_path} does not exist')


def path_must_exist(my_path: str | Path):
    p = Path(my_path).absolute()
    if not p.exists():
        raise PathDoesNotExist(my_path)
    return p


def path_listdir(my_path: str | Path) -> Iterable[Path]:
    my_path = Path(my_path)
    for p in os.listdir(my_path):
        yield my_path / p

