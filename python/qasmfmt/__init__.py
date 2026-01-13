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
    $ qasmfmt --write file.qasm  # format in place
    $ qasmfmt --check file.qasm  # check if formatted
"""

from qasmfmt._qasmfmt import (
    __version__,
    check_file,
    format_file,
    format_str,
)

__all__ = [
    "__version__",
    "format_str",
    "format_file",
    "check_file",
    "main",
]


def main() -> int:
    """CLI entry point."""
    import argparse
    import sys
    from pathlib import Path

    parser = argparse.ArgumentParser(
        prog="qasmfmt",
        description="OpenQASM 3.0 formatter",
    )
    parser.add_argument(
        "files",
        nargs="*",
        type=Path,
        help="Files to format (reads from stdin if not provided)",
    )
    parser.add_argument(
        "-w", "--write",
        action="store_true",
        help="Write formatted output back to files",
    )
    parser.add_argument(
        "-c", "--check",
        action="store_true",
        help="Check if files are formatted (exit 1 if not)",
    )
    parser.add_argument(
        "-d", "--diff",
        action="store_true",
        help="Show diff of formatting changes",
    )
    parser.add_argument(
        "-i", "--indent",
        type=int,
        default=4,
        help="Indent size (default: 4)",
    )
    parser.add_argument(
        "--max-width",
        type=int,
        default=100,
        help="Max line width (default: 100)",
    )
    parser.add_argument(
        "-V", "--version",
        action="version",
        version=f"%(prog)s {__version__}",
    )

    args = parser.parse_args()

    if not args.files:
        source = sys.stdin.read()
        try:
            formatted = format_str(
                source, indent_size=args.indent, max_width=args.max_width
            )
            print(formatted, end="")
            return 0
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            return 1

    exit_code = 0
    for path in args.files:
        if not path.exists():
            print(f"Error: {path} not found", file=sys.stderr)
            exit_code = 1
            continue

        try:
            source = path.read_text()
            formatted = format_str(
                source, indent_size=args.indent, max_width=args.max_width
            )

            if args.check:
                if source != formatted:
                    print(f"Would reformat: {path}", file=sys.stderr)
                    exit_code = 1
            elif args.diff:
                if source != formatted:
                    _print_diff(source, formatted, path)
            elif args.write:
                if source != formatted:
                    path.write_text(formatted)
                    print(f"Formatted: {path}", file=sys.stderr)
            else:
                print(formatted, end="")

        except Exception as e:
            print(f"Error processing {path}: {e}", file=sys.stderr)
            exit_code = 1

    return exit_code


def _print_diff(original: str, formatted: str, path) -> None:
    """Print unified diff between original and formatted."""
    import difflib

    diff = difflib.unified_diff(
        original.splitlines(keepends=True),
        formatted.splitlines(keepends=True),
        fromfile=str(path),
        tofile=str(path),
    )
    for line in diff:
        print(line, end="")


if __name__ == "__main__":
    raise SystemExit(main())
