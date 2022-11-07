import logging
from dataclasses import dataclass


class CustomFormatter(logging.Formatter):
    def __init__(self, fmt):
        super().__init__()
        self.fmt = fmt
        self.FORMATS = {
            logging.DEBUG: Color.cyan + self.fmt + Color.reset,
            logging.INFO: Color.grey + self.fmt + Color.reset,
            logging.WARNING: Color.yellow + self.fmt + Color.reset,
            logging.ERROR: Color.red + self.fmt + Color.reset,
            logging.CRITICAL: Color.bold_red + self.fmt + Color.reset
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


def init_logging(loglevel: int):
    print(loglevel)
    # Create custom logger logging all five levels
    logger = logging.getLogger(__name__)
    logger.setLevel(loglevel)

    # Define format for logs
    fmt = '%(message)s'

    # Create stdout handler for logging to the console (logs all five levels)
    stdout_handler = logging.StreamHandler()
    stdout_handler.setLevel(loglevel)
    stdout_handler.setFormatter(CustomFormatter(fmt))

    # Add both handlers to the logger
    logger.addHandler(stdout_handler)
    logging.info("kek")

