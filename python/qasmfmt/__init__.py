"""OpenQASM 3.0 formatter.

A fast OpenQASM 3.0 formatter written in Rust.

Usage:
    # As a library
    import qasmfmt

    source = '''
    OPENQASM 3.0;
    qubit[2]q;
    h q[0];
    '''
    formatted = qasmfmt.format_str(source)

    # As a CLI
    $ qasmfmt file.qasm          # print formatted output
    $ qasmfmt -w file.qasm       # format in place
    $ qasmfmt --check file.qasm  # check if formatted
    $ qasmfmt --diff file.qasm   # show diff
"""

from __future__ import annotations

import sys
from pathlib import Path
from typing import TYPE_CHECKING

from qasmfmt._qasmfmt import (
    __version__,
    check_file,
    format_file,
    format_str,
)

if TYPE_CHECKING:
    from argparse import Namespace

__all__ = [
    "__version__",
    "format_str",
    "format_file",
    "check_file",
    "main",
]

# Exit codes
EXIT_SUCCESS = 0
EXIT_ERROR = 1
EXIT_USAGE_ERROR = 2

CONFIG_FILENAME = "qasmfmt.toml"


def main() -> int:
    """CLI entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        prog="qasmfmt",
        description="OpenQASM 3.0 formatter",
    )
    parser.add_argument(
        "paths",
        nargs="*",
        help="Files or directories to format. Use '-' for stdin.",
    )
    parser.add_argument(
        "-w",
        "--write",
        action="store_true",
        help="Write formatted output back to files (in-place)",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Check if files are formatted (exit 1 if not)",
    )
    parser.add_argument(
        "--diff",
        action="store_true",
        help="Show unified diff of formatting changes (exit 1 if diff exists)",
    )
    parser.add_argument(
        "-i",
        "--indent",
        type=int,
        default=None,
        help="Indentation size in spaces (default: 4)",
    )
    parser.add_argument(
        "--max-width",
        type=int,
        default=None,
        help="Maximum line width (default: 100)",
    )
    parser.add_argument(
        "--stdin-filename",
        type=Path,
        metavar="PATH",
        help="Virtual filename for stdin input (used for config lookup and error messages)",
    )
    parser.add_argument(
        "--config",
        type=Path,
        metavar="PATH",
        help="Path to configuration file",
    )
    parser.add_argument(
        "--no-config",
        action="store_true",
        help="Disable automatic configuration file discovery",
    )
    parser.add_argument(
        "-V",
        "--version",
        action="version",
        version=f"%(prog)s {__version__}",
    )

    args = parser.parse_args()

    # Validate mutually exclusive modes
    mode_count = sum([args.write, args.check, args.diff])
    if mode_count > 1:
        print(
            "error: --write, --check, and --diff are mutually exclusive",
            file=sys.stderr,
        )
        return EXIT_USAGE_ERROR

    # Determine mode
    if args.write:
        mode = "write"
    elif args.check:
        mode = "check"
    elif args.diff:
        mode = "diff"
    else:
        mode = "print"

    # Check stdin + write combination
    has_stdin = "-" in args.paths or (not args.paths and not sys.stdin.isatty())
    if has_stdin and mode == "write":
        print("error: cannot use --write with stdin input", file=sys.stderr)
        return EXIT_USAGE_ERROR

    # Load configuration
    config = _load_config(args)

    # Apply CLI overrides
    indent_size = args.indent if args.indent is not None else config.get("indent_size", 4)
    max_width = args.max_width if args.max_width is not None else config.get("max_width", 100)

    # Process input
    if not args.paths:
        # No paths: read from stdin if pipe
        if not sys.stdin.isatty():
            return _format_stdin(indent_size, max_width, args.stdin_filename, mode)
        else:
            print("error: no input files provided", file=sys.stderr)
            return EXIT_ERROR

    has_error = False
    has_diff = False

    for path_str in args.paths:
        if path_str == "-":
            # Explicit stdin
            result = _format_stdin(indent_size, max_width, args.stdin_filename, mode)
            if result == EXIT_ERROR:
                has_error = True
            elif result == EXIT_SUCCESS and mode in ("check", "diff"):
                # Check if there was a diff (indicated by special return)
                pass
            has_diff |= result == 99  # Special marker for diff found
        else:
            path = Path(path_str)
            if path.is_dir():
                # Directory: recursively process .qasm files
                for qasm_file in path.rglob("*.qasm"):
                    result, diff = _format_file(qasm_file, indent_size, max_width, mode)
                    if result == EXIT_ERROR:
                        has_error = True
                    has_diff |= diff
            elif path.is_file():
                result, diff = _format_file(path, indent_size, max_width, mode)
                if result == EXIT_ERROR:
                    has_error = True
                has_diff |= diff
            else:
                print(f"Error: {path} not found", file=sys.stderr)
                has_error = True

    if has_error:
        return EXIT_ERROR

    if has_diff and mode in ("check", "diff"):
        return EXIT_ERROR

    return EXIT_SUCCESS


def _load_config(args: Namespace) -> dict:
    """Load configuration from file."""
    if args.no_config:
        return {}

    if args.config:
        return _parse_config_file(args.config)

    # Auto-discover config file
    if args.stdin_filename:
        search_start = args.stdin_filename.parent or Path(".")
    elif args.paths:
        first_path = args.paths[0]
        if first_path == "-":
            search_start = Path(".")
        else:
            p = Path(first_path)
            if p.is_dir():
                search_start = p
            else:
                search_start = p.parent or Path(".")
    else:
        search_start = Path(".")

    config_path = _find_config_file(search_start)
    if config_path:
        return _parse_config_file(config_path)

    return {}


def _find_config_file(start: Path) -> Path | None:
    """Search for qasmfmt.toml from the given directory upward."""
    current = start.resolve()

    while True:
        config_path = current / CONFIG_FILENAME
        if config_path.exists():
            return config_path

        parent = current.parent
        if parent == current:
            break
        current = parent

    return None


def _parse_config_file(path: Path) -> dict:
    """Parse a TOML configuration file."""
    try:
        import tomllib
    except ImportError:
        import tomli as tomllib  # type: ignore[import-not-found,no-redef]

    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception as e:
        print(f"Error reading config file {path}: {e}", file=sys.stderr)
        return {}


def _format_stdin(
    indent_size: int, max_width: int, filename: Path | None, mode: str
) -> int:
    """Format input from stdin."""
    try:
        source = sys.stdin.read()
        formatted = format_str(source, indent_size=indent_size, max_width=max_width)

        display_path = filename or Path("<stdin>")

        if mode in ("print", "write"):
            print(formatted, end="")
            return EXIT_SUCCESS
        elif mode == "check":
            if source != formatted:
                print(f"Would reformat: {display_path}", file=sys.stderr)
                return 99  # Special marker for diff found
            return EXIT_SUCCESS
        elif mode == "diff":
            if source != formatted:
                _print_diff(source, formatted, display_path)
                return 99  # Special marker for diff found
            return EXIT_SUCCESS

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return EXIT_ERROR

    return EXIT_SUCCESS


def _format_file(path: Path, indent_size: int, max_width: int, mode: str) -> tuple[int, bool]:
    """Format a file. Returns (exit_code, has_diff)."""
    try:
        source = path.read_text()
        formatted = format_str(source, indent_size=indent_size, max_width=max_width)

        if mode == "print":
            print(formatted, end="")
            return EXIT_SUCCESS, False
        elif mode == "write":
            if source != formatted:
                path.write_text(formatted)
                print(f"Formatted: {path}", file=sys.stderr)
            return EXIT_SUCCESS, False
        elif mode == "check":
            if source != formatted:
                print(f"Would reformat: {path}", file=sys.stderr)
                return EXIT_SUCCESS, True
            return EXIT_SUCCESS, False
        elif mode == "diff":
            if source != formatted:
                _print_diff(source, formatted, path)
                return EXIT_SUCCESS, True
            return EXIT_SUCCESS, False

    except Exception as e:
        print(f"Error processing {path}: {e}", file=sys.stderr)
        return EXIT_ERROR, False

    return EXIT_SUCCESS, False


def _print_diff(original: str, formatted: str, path: Path) -> None:
    """Print unified diff between original and formatted."""
    import difflib

    diff = list(
        difflib.unified_diff(
            original.splitlines(keepends=True),
            formatted.splitlines(keepends=True),
            fromfile=str(path),
            tofile=str(path),
        )
    )
    if diff:
        for line in diff:
            print(line, end="")


if __name__ == "__main__":
    raise SystemExit(main())
