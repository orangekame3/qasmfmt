"""OpenQASM 3.0 formatter.

This package is currently a placeholder. The full implementation
will provide Rust-powered formatting for OpenQASM 3.0 quantum
circuit files.

Usage (planned):
    from qasmfmt import format_qasm

    source = '''
    OPENQASM 3.0;
    qubit[2]q;
    h q[0];
    '''

    formatted = format_qasm(source)
"""

__version__ = "0.0.1"


def format_qasm(source: str) -> str:
    """Format OpenQASM 3.0 source code.

    Args:
        source: OpenQASM 3.0 source code string.

    Returns:
        Formatted source code.

    Raises:
        NotImplementedError: This is a placeholder implementation.
    """
    raise NotImplementedError(
        "qasmfmt is currently a placeholder. "
        "Full implementation coming soon. "
        "See https://github.com/orangekame3/qasmfmt"
    )


def is_formatted(source: str) -> bool:
    """Check if source code is already formatted.

    Args:
        source: OpenQASM 3.0 source code string.

    Returns:
        True if the source is already formatted.

    Raises:
        NotImplementedError: This is a placeholder implementation.
    """
    raise NotImplementedError(
        "qasmfmt is currently a placeholder. "
        "Full implementation coming soon. "
        "See https://github.com/orangekame3/qasmfmt"
    )
