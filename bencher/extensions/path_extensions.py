import os
from pathlib import Path
from typing import Union, Iterable


class PathDoesNotExist(RuntimeError):
    def __init__(self, path: Union[str, Path]) -> None:
        super().__init__(
            f'Path {path} '
            f'does not exist'
        )


def path_must_exist(path: Union[str, Path]):
    p = Path(path).absolute()
    if not p.exists():
        raise PathDoesNotExist(path)


def path_listdir(path: Union[str, Path]) -> Iterable[Path]:
    path = Path(path)
    for p in os.listdir(path):
        yield path / p
