import argparse
import logging
import os
import sys

from export import generate_bar_graph
from generators.data_to_sort_generator import generate_data_to_sort
from generators.matrix_generator import generate_matrices
from extensions.logging_extensions import init_logging, LOGGER
from generators.prime_number_count_generator import generate_prime_number_count
from pipeline import Pipeline, get_pipeline, get_all_pipeline_names
from tests.settings import DATA_DIR
from tests.tests_config_data import TestsConfigData, get_tests_config_data
from tests.tests_config import TestsConfig


def parse_args(arguments: list):
    parser = argparse.ArgumentParser(description="FPLB - Flexible Programming Language Benchmarking"
                                                 "Tool or comparing the speed of programming languages")
    parser.add_argument("-p", "--pipeline",
                        type=str,
                        required=True,
                        choices=get_all_pipeline_names(),
                        help="Pipeline for testing")
    parser.add_argument("-o", "--output-file",
                        type=str,
                        help="Path to file with test result")
    parser.add_argument("-i", "--image",
                        action="store_true",
                        help="Creates an image graph.jpeg with a graph comparing execution speeds")
    parser.add_argument("-g", "--generate",
                        action="store_true",
                        help="Generates a new data for tests")
    parser.add_argument("-c", "--count",
                        type=int,
                        default=1,
                        help="Number of time to repeat tests")
    parser.add_argument("-d", "--debug",
                        action="store_true",
                        help="Enables debug mode")
    return parser.parse_args(arguments)


def main(raw_arguments: list) -> None:
    args = parse_args(raw_arguments[1:])

    if args.debug:
        init_logging(logging.DEBUG)
    else:
        init_logging(logging.INFO)

    is_data_exists: bool = DATA_DIR.exists()
    if args.generate or not is_data_exists:
        if not is_data_exists:
            os.mkdir(DATA_DIR)
        LOGGER.info("Generating new data")
        generate_prime_number_count()
        generate_matrices()
        generate_data_to_sort()
        LOGGER.info("New data successfully generated")

    pipeline: Pipeline = get_pipeline(args.pipeline)
    LOGGER.info(f"Pipeline name -> {pipeline.name}")
    LOGGER.info(f"Pipeline description -> {pipeline.description}")
    LOGGER.info(f"Number of repeats -> {args.count}")
    pipeline.print_pipeline()
    tests_config_data: TestsConfigData = get_tests_config_data()
    tests_config: TestsConfig = TestsConfig.data_to_tests_config(tests_config_data)
    tests_config.exec_pipeline(pipeline, args.count)

    if args.image:
        generate_bar_graph(pipeline, tests_config)


if __name__ == '__main__':
    main(sys.argv)
