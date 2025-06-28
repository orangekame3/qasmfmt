package cmd

import (
	"strings"
	"testing"
)

func TestFormatQASMWithConfig(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		config   *Config
		expected string
	}{
		{
			name:  "default indentation",
			input: "OPENQASM 3.0;\nqubit q;",
			config: &Config{
				Indent:  2,
				Newline: true,
			},
			expected: "OPENQASM 3.0;\nqubit q;\n",
		},
		{
			name:  "4-space indentation",
			input: "OPENQASM 3.0;\nqubit q;",
			config: &Config{
				Indent:  4,
				Newline: true,
			},
			expected: "OPENQASM 3.0;\nqubit q;\n",
		},
		{
			name:  "no trailing newline",
			input: "OPENQASM 3.0;\nqubit q;",
			config: &Config{
				Indent:  2,
				Newline: false,
			},
			expected: "OPENQASM 3.0;\nqubit q;",
		},
		{
			name:  "8-space indentation",
			input: "OPENQASM 3.0;\nqubit q;",
			config: &Config{
				Indent:  8,
				Newline: true,
			},
			expected: "OPENQASM 3.0;\nqubit q;\n",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := FormatQASMWithConfig(tt.input, tt.config)
			if err != nil {
				t.Fatalf("FormatQASMWithConfig() error = %v", err)
			}
			if result != tt.expected {
				t.Errorf("FormatQASMWithConfig() = %q, want %q", result, tt.expected)

				// Show detailed diff
				resultLines := strings.Split(result, "\n")
				expectedLines := strings.Split(tt.expected, "\n")

				t.Logf("Result lines: %d", len(resultLines))
				for i, line := range resultLines {
					t.Logf("  [%d]: %q", i, line)
				}

				t.Logf("Expected lines: %d", len(expectedLines))
				for i, line := range expectedLines {
					t.Logf("  [%d]: %q", i, line)
				}
			}
		})
	}
}

func TestNewFormatterWithConfig(t *testing.T) {
	tests := []struct {
		name            string
		config          *Config
		expectedIndent  int
		expectedNewline bool
	}{
		{
			name: "default config",
			config: &Config{
				Indent:  2,
				Newline: true,
			},
			expectedIndent:  2,
			expectedNewline: true,
		},
		{
			name: "4-space indentation",
			config: &Config{
				Indent:  4,
				Newline: true,
			},
			expectedIndent:  4,
			expectedNewline: true,
		},
		{
			name: "no trailing newline",
			config: &Config{
				Indent:  2,
				Newline: false,
			},
			expectedIndent:  2,
			expectedNewline: false,
		},
		{
			name: "8-space indentation, no newline",
			config: &Config{
				Indent:  8,
				Newline: false,
			},
			expectedIndent:  8,
			expectedNewline: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			formatter := NewFormatterWithConfig(tt.config)

			if formatter.indentSize != tt.expectedIndent {
				t.Errorf("NewFormatterWithConfig() indentSize = %d, want %d",
					formatter.indentSize, tt.expectedIndent)
			}

			if formatter.newline != tt.expectedNewline {
				t.Errorf("NewFormatterWithConfig() newline = %v, want %v",
					formatter.newline, tt.expectedNewline)
			}
		})
	}
}
