import sys
from pathlib import Path

from export import generate_bar_graph
from pipeline import Pipeline
from tests.tests_config import TestsConfig
from pipeline import get_pipeline


def main(raw_arguments: list) -> None:
    pipeline_name: str = raw_arguments[1]
    pipeline: Pipeline = get_pipeline(pipeline_name)

    tests_config: TestsConfig = TestsConfig.results_to_test_config(Path("output"))

    generate_bar_graph(pipeline, tests_config)


if __name__ == '__main__':
    main(sys.argv)
