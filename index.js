/**
 * OpenQASM 3.0 formatter
 *
 * This package is currently a placeholder. The full implementation
 * will provide Rust-powered (via WASM) formatting for OpenQASM 3.0
 * quantum circuit files.
 *
 * @module qasmfmt
 */

"use strict";

/**
 * Format OpenQASM 3.0 source code.
 *
 * @param {string} source - OpenQASM 3.0 source code
 * @returns {string} Formatted source code
 * @throws {Error} This is a placeholder implementation
 *
 * @example
 * const { formatQasm } = require('qasmfmt');
 *
 * const source = `
 * OPENQASM 3.0;
 * qubit[2]q;
 * h q[0];
 * `;
 *
 * const formatted = formatQasm(source);
 */
function formatQasm(source) {
  throw new Error(
    "qasmfmt is currently a placeholder. " +
      "Full implementation coming soon. " +
      "See https://github.com/orangekame3/qasmfmt"
  );
}

/**
 * Check if source code is already formatted.
 *
 * @param {string} source - OpenQASM 3.0 source code
 * @returns {boolean} True if the source is already formatted
 * @throws {Error} This is a placeholder implementation
 */
function isFormatted(source) {
  throw new Error(
    "qasmfmt is currently a placeholder. " +
      "Full implementation coming soon. " +
      "See https://github.com/orangekame3/qasmfmt"
  );
}

module.exports = {
  formatQasm,
  isFormatted,
};
