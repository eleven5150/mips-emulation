import logging
from dataclasses import dataclass

LOGGER = logging.getLogger("FPLB")


class CustomFormatter(logging.Formatter):
    def __init__(self, fmt):
        super().__init__()
        self.fmt = fmt
        self.FORMATS = {
            logging.DEBUG: Color.debug(self.fmt),
            logging.INFO: Color.info(self.fmt),
            logging.WARNING: Color.warning(self.fmt),
            logging.ERROR: Color.error(self.fmt),
            logging.CRITICAL: Color.critical(self.fmt)
        }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)


@dataclass
class Color:
    grey = '\x1b[38;21m'
    cyan = '\033[96m'
    green = '\033[92m'
    yellow = '\033[93m'
    red = '\033[91m'
    bold_red = '\x1b[31;1m'

    reset = '\033[0m'

    @classmethod
    def debug(cls, string: str) -> str:
        return f"{cls.cyan}{string}{cls.reset}"

    @classmethod
    def info(cls, string: str) -> str:
        return f"{cls.grey}{string}{cls.reset}"

    @classmethod
    def warning(cls, string: str) -> str:
        return f"{cls.yellow}{string}{cls.reset}"

    @classmethod
    def error(cls, string: str) -> str:
        return f"{cls.red}{string}{cls.reset}"

    @classmethod
    def critical(cls, string: str) -> str:
        return f"{cls.bold_red}{string}{cls.reset}"


def init_logging(loglevel: int) -> None:
    LOGGER.setLevel(loglevel)

    fmt = '%(message)s'

    stdout_handler = logging.StreamHandler()
    stdout_handler.setLevel(loglevel)
    stdout_handler.setFormatter(CustomFormatter(fmt))

    LOGGER.addHandler(stdout_handler)

