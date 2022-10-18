import os
from pathlib import Path
from typing import Union, Iterable

# Didn't find how to retrieve this string in-runtime
MODULE_NAME = 'extensions.'


def get_root_directory() -> Path:
    # Module directory
    file_path = Path(__file__).absolute().parent

    for i in range(MODULE_NAME.count('.')):
        file_path = file_path.parent

    return file_path



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
