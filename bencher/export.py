from pathlib import Path

from matplotlib import pyplot as plt

from extensions.path_extensions import path_safe_mkdir, get_root_directory
from pipeline import Pipeline, NOT_TEST_PIPELINES, NotTestPipelineException
from tests.tests_config import TestsConfig


def generate_bar_graph(pipeline: Pipeline, tests_config: TestsConfig) -> None:
    def addlabels(x, y):
        for i in range(len(x)):
            plt.text(i, y[i], y[i], ha="center")

    if pipeline.test in NOT_TEST_PIPELINES:
        raise NotTestPipelineException(f"Can't generate graph for this pipeline {pipeline.name}")

    plt.rcdefaults()
    languages: list[str] = list(pipeline.pipeline.keys())

    results: dict[str, float] = dict()
    for lang in languages:
        results.update({lang: tests_config.get_result_by_lang_and_test(lang, pipeline.test)})

    results = dict(sorted(results.items(), key=lambda item: item[1]))

    plt.bar(results.keys(), results.values(), align="center")
    addlabels(results.keys(), list(results.values()))

    plt.xlabel("Programming language")
    plt.xticks(rotation=30)
    plt.ylabel("Time, s")
    plt.title(f"Results for {pipeline.test}")
    plt.tight_layout()
    output_dir: Path = Path(get_root_directory() / "output")
    path_safe_mkdir(output_dir)
    plt.savefig(Path(output_dir / "output.jpg"), dpi=600)
