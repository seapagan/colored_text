"""Regression tests for the optional local complexity checker."""

from __future__ import annotations

import importlib.util
import os
import re
import sys
import tempfile
import unittest
from pathlib import Path

# ``Any`` is referenced through a string annotation below. Codacy's Pylint
# analysis does not count that as usage, so this narrow suppression is required;
# Ruff and mypy need no equivalent suppression.
from typing import TYPE_CHECKING, Any, cast  # pylint: disable=unused-import
from unittest.mock import patch

if TYPE_CHECKING:
    from types import ModuleType


def _load_checker() -> ModuleType:
    path = Path(__file__).parents[1] / "scripts" / "check_complexity.py"
    spec = importlib.util.spec_from_file_location("check_complexity", path)
    if spec is None or spec.loader is None:
        message = f"cannot load {path}"
        raise RuntimeError(message)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


CHECKER = cast("Any", _load_checker())


VALID_XML = """\
<root><measure type="File"><labels><label>NCSS</label><label>CCN</label></labels>
<item name="src/main.rs"><value>7</value><value>2</value></item>
</measure></root>
"""

INVALID_XML_CASES = (
    ("<root>", "invalid Lizard XML:"),
    ("<root />", "expected one XML File measure, found 0"),
    (
        '<root><measure type="File"/><measure type="File"/></root>',
        "expected one XML File measure, found 2",
    ),
    (
        (
            '<root><measure type="File"><labels><label>CCN</label></labels>'
            "</measure></root>"
        ),
        "XML File measure lacks NCSS: ['CCN']",
    ),
    (
        (
            '<root><measure type="File"><labels><label>NCSS</label><label>CCN</label>'
            '</labels><item name="src/main.rs"><value>7</value></item></measure></root>'
        ),
        "unexpected XML file record for 'src/main.rs'",
    ),
    (
        (
            '<root><measure type="File"><labels><label>NCSS</label></labels>'
            '<item name="src/main.rs"><value>7</value></item>'
            '<item name="src/main.rs"><value>8</value></item></measure></root>'
        ),
        "unexpected XML file record for 'src/main.rs'",
    ),
    (
        (
            '<root><measure type="File"><labels><label>NCSS</label></labels>'
            '<item name="src/main.rs"><value>many</value></item></measure></root>'
        ),
        "invalid XML NCSS value for 'src/main.rs': 'many'",
    ),
    (
        (
            '<root><measure type="File"><labels><label>NCSS</label></labels>'
            '<item name="src/main.rs"><value>-1</value></item></measure></root>'
        ),
        "invalid XML NCSS value for 'src/main.rs': '-1'",
    ),
)


class ComplexityCheckerTests(unittest.TestCase):
    """Exercise checker configuration, parsing, and source analysis."""

    def test_source_files_include_non_ignored_rust_and_python(self) -> None:
        """Git discovery includes both source types and respects ignores."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            previous = Path.cwd()
            try:
                os.chdir(root)
                CHECKER._run(["git", "init", "-q"])
                (root / ".gitignore").write_text(".venv/\n__pycache__/\n")
                (root / "tracked.rs").write_text("fn main() {}\n")
                (root / "tracked.py").write_text("pass\n")
                (root / "untracked.rs").write_text("fn helper() {}\n")
                (root / "untracked.py").write_text("pass\n")
                (root / "ignored.txt").write_text("not source\n")
                (root / ".venv").mkdir()
                (root / ".venv" / "ignored.py").write_text("pass\n")
                CHECKER._run(["git", "add", ".gitignore", "tracked.rs", "tracked.py"])
                files = CHECKER._source_files()
            finally:
                os.chdir(previous)

        self.assertEqual(
            files,
            ["tracked.py", "tracked.rs", "untracked.py", "untracked.rs"],
        )

    def test_source_files_exclude_unstaged_deletions_and_renames(self) -> None:
        """Git discovery returns only source files present in the worktree."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            previous = Path.cwd()
            try:
                os.chdir(root)
                CHECKER._run(["git", "init", "-q"])
                deleted = root / "deleted.py"
                old = root / "old.py"
                deleted.write_text("pass\n")
                old.write_text("pass\n")
                CHECKER._run(["git", "add", "deleted.py", "old.py"])
                deleted.unlink()
                old.rename(root / "new.py")
                files = CHECKER._source_files()
            finally:
                os.chdir(previous)

        self.assertEqual(files, ["new.py"])

    def test_source_files_deduplicate_conflicted_index_entries(self) -> None:
        """A conflicted source appears once despite its three index stages."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            previous = Path.cwd()
            try:
                os.chdir(root)
                CHECKER._run(["git", "init", "-q", "--initial-branch=main"])
                CHECKER._run(["git", "config", "user.email", "test@example.com"])
                CHECKER._run(["git", "config", "user.name", "Test User"])
                CHECKER._run(["git", "config", "commit.gpgsign", "false"])
                source = root / "conflicted.rs"
                source.write_text("fn value() -> i32 { 0 }\n")
                CHECKER._run(["git", "add", "conflicted.rs"])
                CHECKER._run(["git", "commit", "-q", "-m", "base"])
                CHECKER._run(["git", "switch", "-q", "-c", "other"])
                source.write_text("fn value() -> i32 { 1 }\n")
                CHECKER._run(["git", "commit", "-qam", "other"])
                CHECKER._run(["git", "switch", "-q", "main"])
                source.write_text("fn value() -> i32 { 2 }\n")
                CHECKER._run(["git", "commit", "-qam", "main"])
                with self.assertRaisesRegex(CHECKER.CheckerError, "git merge failed"):
                    CHECKER._run(["git", "merge", "other"])

                stages = CHECKER._run(["git", "ls-files", "-u", "--", "conflicted.rs"])
                files = CHECKER._source_files()
            finally:
                os.chdir(previous)

        self.assertEqual(len(stages.splitlines()), 3)
        self.assertEqual(files, ["conflicted.rs"])

    @unittest.skipIf(os.name == "nt", "POSIX filenames may contain arbitrary bytes")
    def test_source_files_round_trip_non_utf8_filename_bytes(self) -> None:
        """Git paths with undecodable bytes survive subprocess decoding."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            previous = Path.cwd()
            filename = os.fsdecode(b"invalid-\xff.py")
            try:
                os.chdir(root)
                CHECKER._run(["git", "init", "-q"])
                try:
                    Path(filename).write_text("pass\n")
                except OSError as error:
                    self.skipTest(f"filesystem rejects non-UTF-8 filenames: {error}")
                files = CHECKER._source_files()
            finally:
                os.chdir(previous)

        self.assertEqual(files, [filename])

    def test_lizard_command_terminates_options_before_source_files(self) -> None:
        """One separator keeps leading-hyphen source paths positional."""
        self.assertEqual(
            CHECKER._lizard_command(
                ["-V", "--csv"],
                ["scripts/check_complexity.py", "-odd.py"],
            ),
            [
                "lizard",
                "-l",
                "rust",
                "-l",
                "python",
                "-i",
                "-1",
                "-V",
                "--csv",
                "--",
                "scripts/check_complexity.py",
                "-odd.py",
            ],
        )

    def test_function_metrics_parse_lizard_1_23_csv(self) -> None:
        """Representative verbose Lizard CSV produces function metrics."""
        output = (
            "NLOC,CCN,token,PARAM,length,location,file,function,long_name,start,end\n"
            "12,4,100,2,20,func@5-24@src/main.rs,src/main.rs,func,func(),5,24\n"
        )

        self.assertEqual(
            CHECKER._function_metrics(output, {"src/main.rs"}),
            [
                CHECKER.FunctionMetric(
                    path="src/main.rs",
                    name="func",
                    line=5,
                    nloc=12,
                    ccn=4,
                    parameters=2,
                ),
            ],
        )

    def test_function_metrics_reject_missing_or_incorrect_columns(self) -> None:
        """The parser rejects CSV without every required named column."""
        outputs = (
            "NLOC,CCN,file,function,start\n12,4,src/main.rs,func,5\n",
            "NLOC,CCN,params,file,function,start\n12,4,2,src/main.rs,func,5\n",
        )
        for output in outputs:
            with (
                self.subTest(output=output),
                self.assertRaisesRegex(
                    CHECKER.CheckerError,
                    "unexpected CSV columns",
                ),
            ):
                CHECKER._function_metrics(output, {"src/main.rs"})

    def test_function_metrics_reject_malformed_numeric_fields(self) -> None:
        """Malformed or out-of-range CSV metrics remain fatal."""
        header = (
            "NLOC,CCN,token,PARAM,length,location,file,function,long_name,start,end"
        )
        cases = (
            ("many,4,100,2,20,loc,src/main.rs,func,func(),5,24", "NLOC"),
            ("12,-1,100,2,20,loc,src/main.rs,func,func(),5,24", "CCN"),
            ("12,4,100,nope,20,loc,src/main.rs,func,func(),5,24", "PARAM"),
            ("12,4,100,2,20,loc,src/main.rs,func,func(),0,24", "start"),
        )
        for row, field in cases:
            with (
                self.subTest(field=field),
                self.assertRaisesRegex(
                    CHECKER.CheckerError,
                    f"invalid CSV {field} value",
                ),
            ):
                CHECKER._function_metrics(f"{header}\n{row}\n", {"src/main.rs"})

    def test_function_metrics_reject_unexpected_source_path(self) -> None:
        """CSV records outside the expected Git source set remain fatal."""
        output = (
            "NLOC,CCN,token,PARAM,length,location,file,function,long_name,start,end\n"
            "12,4,100,2,20,func@5-24@other.rs,other.rs,func,func(),5,24\n"
        )

        with self.assertRaisesRegex(
            CHECKER.CheckerError,
            "unexpected CSV function record",
        ):
            CHECKER._function_metrics(output, {"src/main.rs"})

    def test_file_metrics_parses_lizard_xml(self) -> None:
        """Valid Lizard XML produces normalized per-file NCSS values."""
        self.assertEqual(
            CHECKER._file_metrics(VALID_XML, {"src/main.rs"}),
            {"src/main.rs": 7},
        )

    def test_file_metrics_rejects_malformed_xml_records(self) -> None:
        """Malformed XML structures and values remain fatal."""
        for output, expected_message in INVALID_XML_CASES:
            with (
                self.subTest(
                    expected_message=expected_message,
                ),
                self.assertRaisesRegex(
                    CHECKER.CheckerError,
                    re.escape(expected_message),
                ),
            ):
                CHECKER._file_metrics(output, {"src/main.rs"})

    def test_file_metrics_reports_missing_and_unexpected_files(self) -> None:
        """Source-set mismatch diagnostics identify both differences."""
        message = (
            "source analysis file mismatch; missing=['src/lib.rs'], "
            "unexpected=['src/main.rs']"
        )
        with self.assertRaisesRegex(CHECKER.CheckerError, re.escape(message)):
            CHECKER._file_metrics(VALID_XML, {"src/lib.rs"})

    def test_collect_findings_uses_strict_greater_than_for_all_metrics(self) -> None:
        """Every configured threshold allows equality and reports one above."""
        thresholds = CHECKER.Thresholds(10, 50, 8, 500)
        equal = CHECKER.FunctionMetric("equal.py", "equal", 1, 50, 10, 8)
        above = CHECKER.FunctionMetric("above.py", "above", 2, 51, 11, 9)

        self.assertEqual(
            CHECKER._collect_findings([equal], {"equal.py": 500}, thresholds),
            [],
        )
        self.assertEqual(
            [
                finding.metric
                for finding in CHECKER._collect_findings(
                    [above],
                    {"above.py": 501},
                    thresholds,
                )
            ],
            ["file NLOC", "CCN", "function NLOC", "parameters"],
        )

    def test_limit_accepts_positive_integer(self) -> None:
        """A positive decimal threshold is parsed as an integer."""
        with patch.dict(os.environ, {"TEST_LIMIT": "17"}):
            self.assertEqual(CHECKER._limit("TEST_LIMIT"), 17)

    def test_limit_rejects_invalid_values(self) -> None:
        """Zero, negative, non-integer, and empty thresholds are rejected."""
        for value in ("0", "-1", "1.5", ""):
            with (
                self.subTest(value=value),
                patch.dict(os.environ, {"TEST_LIMIT": value}),
                self.assertRaisesRegex(
                    CHECKER.CheckerError,
                    re.escape(f"TEST_LIMIT must be a positive integer, got {value!r}"),
                ),
            ):
                CHECKER._limit("TEST_LIMIT")

    def test_main_rejects_lizard_version_mismatch(self) -> None:
        """The checker enforces exact equality with the configured version."""
        environment = {
            "COMPLEXITY_LIZARD_VERSION": "1.23.0",
            "COMPLEXITY_MAX_CCN": "10",
            "COMPLEXITY_MAX_FUNCTION_NLOC": "50",
            "COMPLEXITY_MAX_PARAMETERS": "8",
            "COMPLEXITY_MAX_FILE_NLOC": "500",
        }
        with (
            patch.dict(os.environ, environment, clear=True),
            patch.object(CHECKER, "_run", return_value="1.24.0\n"),
            self.assertRaisesRegex(
                CHECKER.CheckerError,
                "Lizard version mismatch: expected 1.23.0, got '1.24.0'",
            ),
        ):
            CHECKER.main()


if __name__ == "__main__":
    unittest.main()
