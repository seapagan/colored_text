"""Report Codacy-aligned source complexity findings from Lizard output."""

from __future__ import annotations

import csv
import io
import os
import re

# Security suppressions below are intentional and tool-specific: Bandit/Codacy
# consumes the B-coded ``nosec`` comments; matching Ruff S-coded suppressions
# remain for equivalent Ruff security rules when enabled. Do not remove either
# without rerunning Bandit/Codacy and Ruff.
# Subprocess use is the reviewed boundary for invoking Git and Lizard.
import subprocess  # nosec B404
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

# XML input is produced by the locally invoked Lizard process.
from xml.etree.ElementTree import Element, ParseError, fromstring  # nosec B405

MINIMUM_PYTHON = (3, 10)

if sys.version_info < MINIMUM_PYTHON:
    print("check_complexity.py requires Python 3.10 or newer", file=sys.stderr)
    raise SystemExit(1)

if TYPE_CHECKING:
    from collections.abc import Sequence


@dataclass(frozen=True)
class Thresholds:
    """Configured upper bounds for each reported complexity metric."""

    ccn: int
    function_nloc: int
    parameters: int
    file_nloc: int


@dataclass(frozen=True)
class FunctionMetric:
    """Lizard metrics for one source function."""

    path: str
    name: str
    line: int
    nloc: int
    ccn: int
    parameters: int


@dataclass(frozen=True, order=True)
class Finding:
    """One deterministically sortable advisory finding."""

    path: str
    line: int
    metric: str
    message: str


class CheckerError(Exception):
    """Raised when the checker cannot produce a trustworthy report."""


def _limit(name: str) -> int:
    value = os.environ.get(name, "")
    if not re.fullmatch(r"[1-9][0-9]*", value):
        message = f"{name} must be a positive integer, got {value!r}"
        raise CheckerError(message)
    return int(value)


def _run(command: Sequence[str]) -> str:
    try:
        # Controlled argv boundary: fixed executable, separate arguments, no shell.
        result = subprocess.run(  # noqa: S603  # nosec B603
            command,
            capture_output=True,
            text=True,
            errors="surrogateescape",
            check=False,
        )
    except FileNotFoundError as error:
        message = f"required executable not found: {command[0]}"
        raise CheckerError(message) from error
    if result.returncode != 0:
        detail = (
            result.stderr.strip() or result.stdout.strip() or "no diagnostic output"
        )
        message = (
            f"{' '.join(command[:2])} failed with exit code "
            f"{result.returncode}: {detail}"
        )
        raise CheckerError(message)
    return result.stdout


def _normalized(path: str) -> str:
    return Path(path.removeprefix("./")).as_posix()


def _csv_text(row: dict[str, str | None], field: str) -> str:
    value = row.get(field)
    if not value:
        message = f"invalid CSV {field} value: {value!r}"
        raise CheckerError(message)
    return value


def _integer(row: dict[str, str | None], field: str) -> int:
    raw_value = row.get(field)
    try:
        value = int(raw_value) if raw_value is not None else -1
    except ValueError as error:
        message = f"invalid CSV {field} value: {raw_value!r}"
        raise CheckerError(message) from error
    if value < 0:
        message = f"invalid CSV {field} value: {raw_value!r}"
        raise CheckerError(message)
    return value


def _source_files() -> list[str]:
    output = _run(
        [
            "git",
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            "*.rs",
            "*.py",
        ],
    )
    files = sorted(
        {_normalized(path) for path in output.split("\0") if Path(path).is_file()}
    )
    if not files:
        message = "no non-ignored Rust or Python source files found"
        raise CheckerError(message)
    return files


def _lizard_command(output_options: Sequence[str], files: Sequence[str]) -> list[str]:
    return [
        "lizard",
        "-l",
        "rust",
        "-l",
        "python",
        "-i",
        "-1",
        *output_options,
        "--",
        *files,
    ]


def _function_metrics(output: str, files: set[str]) -> list[FunctionMetric]:
    reader = csv.DictReader(io.StringIO(output))
    required = {"NLOC", "CCN", "PARAM", "file", "function", "start"}
    if reader.fieldnames is None or not required.issubset(reader.fieldnames):
        message = f"unexpected CSV columns: {reader.fieldnames!r}"
        raise CheckerError(message)

    metrics = []
    for row in reader:
        path = _normalized(_csv_text(row, "file"))
        name = _csv_text(row, "function")
        if path not in files:
            message = f"unexpected CSV function record: {row!r}"
            raise CheckerError(message)
        start = _integer(row, "start")
        if start == 0:
            message = f"invalid CSV start value: {start}"
            raise CheckerError(message)
        metrics.append(
            FunctionMetric(
                path=path,
                name=name,
                line=start,
                nloc=_integer(row, "NLOC"),
                ccn=_integer(row, "CCN"),
                parameters=_integer(row, "PARAM"),
            ),
        )
    return metrics


def _xml_file_measure(
    output: str,
) -> tuple[Element, list[str | None], int]:
    try:
        # Lizard's reported version is checked before its output is parsed here.
        root = fromstring(output)  # noqa: S314  # nosec B314
    except ParseError as error:
        message = f"invalid Lizard XML: {error}"
        raise CheckerError(message) from error

    measures = root.findall(".//measure[@type='File']")
    if len(measures) != 1:
        message = f"expected one XML File measure, found {len(measures)}"
        raise CheckerError(message)
    measure = measures[0]
    labels = [label.text for label in measure.findall("./labels/label")]
    if "NCSS" not in labels:
        message = f"XML File measure lacks NCSS: {labels!r}"
        raise CheckerError(message)
    return measure, labels, labels.index("NCSS")


def _xml_ncss(path: str, raw_nloc: str | None) -> int:
    try:
        nloc = int(raw_nloc) if raw_nloc is not None else -1
    except ValueError as error:
        message = f"invalid XML NCSS value for {path!r}: {raw_nloc!r}"
        raise CheckerError(message) from error
    if nloc < 0:
        message = f"invalid XML NCSS value for {path!r}: {raw_nloc!r}"
        raise CheckerError(message)
    return nloc


def _xml_file_record(
    item: Element,
    labels: Sequence[str | None],
    nloc_index: int,
) -> tuple[str, int]:
    path = _normalized(item.get("name", ""))
    values = [value.text for value in item.findall("./value")]
    if not path or len(values) != len(labels):
        message = f"unexpected XML file record for {path!r}"
        raise CheckerError(message)
    return path, _xml_ncss(path, values[nloc_index])


def _file_metrics(output: str, expected_files: set[str]) -> dict[str, int]:
    measure, labels, nloc_index = _xml_file_measure(output)

    metrics: dict[str, int] = {}
    for item in measure.findall("./item"):
        path, nloc = _xml_file_record(item, labels, nloc_index)
        if path in metrics:
            message = f"unexpected XML file record for {path!r}"
            raise CheckerError(message)
        metrics[path] = nloc

    actual_files = set(metrics)
    if actual_files != expected_files:
        missing = sorted(expected_files - actual_files)
        unexpected = sorted(actual_files - expected_files)
        message = (
            f"source analysis file mismatch; missing={missing!r}, "
            f"unexpected={unexpected!r}"
        )
        raise CheckerError(message)
    return metrics


def _collect_findings(
    functions: Sequence[FunctionMetric],
    file_nloc: dict[str, int],
    thresholds: Thresholds,
) -> list[Finding]:
    findings = []
    for function in functions:
        for metric, value, configured in (
            ("CCN", function.ccn, thresholds.ccn),
            ("function NLOC", function.nloc, thresholds.function_nloc),
            ("parameters", function.parameters, thresholds.parameters),
        ):
            if value > configured:
                message = (
                    f"{function.path}:{function.line}: function {function.name!r}: "
                    f"{metric} actual={value} limit={configured}"
                )
                findings.append(Finding(function.path, function.line, metric, message))
    for path, value in file_nloc.items():
        if value > thresholds.file_nloc:
            message = f"{path}: file NLOC actual={value} limit={thresholds.file_nloc}"
            findings.append(Finding(path, 0, "file NLOC", message))
    return sorted(findings)


def _report(
    version: str,
    thresholds: Thresholds,
    findings: Sequence[Finding],
) -> None:
    print(f"Lizard version: {version}")
    print(
        "Effective thresholds: "
        f"CCN <= {thresholds.ccn}; "
        f"function NLOC <= {thresholds.function_nloc}; "
        f"parameters <= {thresholds.parameters}; "
        f"file NLOC <= {thresholds.file_nloc}",
    )
    for finding in findings:
        print(finding.message)
    if findings:
        print(
            f"Complexity: {len(findings)} advisory finding(s); "
            "findings do not fail verification.",
        )
    else:
        print("Complexity: no advisory findings.")


def main() -> None:
    """Run the complexity checker and print its advisory report."""
    expected_version = os.environ.get("COMPLEXITY_LIZARD_VERSION", "")
    if not expected_version:
        message = "COMPLEXITY_LIZARD_VERSION must not be empty"
        raise CheckerError(message)
    thresholds = Thresholds(
        ccn=_limit("COMPLEXITY_MAX_CCN"),
        function_nloc=_limit("COMPLEXITY_MAX_FUNCTION_NLOC"),
        parameters=_limit("COMPLEXITY_MAX_PARAMETERS"),
        file_nloc=_limit("COMPLEXITY_MAX_FILE_NLOC"),
    )

    actual_version = _run(["lizard", "--version"]).strip()
    if actual_version != expected_version:
        message = (
            f"Lizard version mismatch: expected {expected_version}, "
            f"got {actual_version!r}"
        )
        raise CheckerError(message)

    files = _source_files()
    functions = _function_metrics(
        _run(_lizard_command(["-V", "--csv"], files)),
        set(files),
    )
    file_nloc = _file_metrics(
        _run(_lizard_command(["--xml"], files)),
        set(files),
    )
    _report(
        actual_version, thresholds, _collect_findings(functions, file_nloc, thresholds)
    )


if __name__ == "__main__":
    try:
        main()
    except CheckerError as error:
        print(f"error: complexity checker: {error}", file=sys.stderr)
        sys.exit(1)
