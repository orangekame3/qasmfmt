package cmd

import (
	"strings"
	"testing"
)

func TestFormatQASM(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic version only",
			input:    "OPENQASM 3.0;",
			expected: "OPENQASM 3.0;\n",
		},
		{
			name:     "version with include",
			input:    "OPENQASM 3.0;\ninclude \"stdgates.qasm\";",
			expected: "OPENQASM 3.0;\ninclude \"stdgates.qasm\";\n",
		},
		{
			name:     "simple qubit declaration",
			input:    "OPENQASM 3.0;\nqubit q;",
			expected: "OPENQASM 3.0;\nqubit q;\n",
		},
		{
			name:     "qubit array declaration",
			input:    "OPENQASM 3.0;\nqubit[2] q;",
			expected: "OPENQASM 3.0;\nqubit[2] q;\n",
		},
		{
			name:     "malformed qubit declaration",
			input:    "OPENQASM 3.0;\nqubit[2]q;",
			expected: "OPENQASM 3.0;\nqubit[2] q;\n",
		},
		{
			name:     "simple gate call",
			input:    "OPENQASM 3.0;\nqubit q;\nh q;",
			expected: "OPENQASM 3.0;\nqubit q;\nh q;\n",
		},
		{
			name:     "malformed gate call",
			input:    "OPENQASM 3.0;\nqubit[2] q;\nhq[0];",
			expected: "OPENQASM 3.0;\nqubit[2] q;\nh q[0];\n",
		},
		{
			name:     "two-qubit gate",
			input:    "OPENQASM 3.0;\nqubit[2] q;\ncx q[0], q[1];",
			expected: "OPENQASM 3.0;\nqubit[2] q;\ncx q[0], q[1];\n",
		},
		{
			name:     "malformed two-qubit gate",
			input:    "OPENQASM 3.0;\nqubit[2] q;\ncxq[0],q[1];",
			expected: "OPENQASM 3.0;\nqubit[2] q;\ncx q[0], q[1];\n",
		},
		{
			name:     "measurement",
			input:    "OPENQASM 3.0;\nqubit q;\nbit c;\nmeasure q -> c;",
			expected: "OPENQASM 3.0;\nqubit q;\nbit c;\nmeasure q -> c;\n",
		},
		{
			name:     "malformed measurement",
			input:    "OPENQASM 3.0;\nqubit q;\nbit c;\nmeasureq->c;",
			expected: "OPENQASM 3.0;\nqubit q;\nbit c;\nmeasure q -> c;\n",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := FormatQASM(tt.input)
			if err != nil {
				t.Fatalf("FormatQASM() error = %v", err)
			}
			if result != tt.expected {
				t.Errorf("FormatQASM() = %q, want %q", result, tt.expected)

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

func TestValidateQASM(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		wantErr bool
	}{
		{
			name:    "valid basic program",
			input:   "OPENQASM 3.0;\nqubit q;",
			wantErr: false,
		},
		{
			name:    "empty content",
			input:   "",
			wantErr: true,
		},
		{
			name:    "only whitespace",
			input:   "   \n  \t  \n  ",
			wantErr: true,
		},
		{
			name:    "valid with gates",
			input:   "OPENQASM 3.0;\nqubit q;\nh q;",
			wantErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := ValidateQASM(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateQASM() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestFormatterMethods(t *testing.T) {
	f := NewFormatter()

	tests := []struct {
		name     string
		method   func(string) string
		input    string
		expected string
	}{
		{
			name:     "formatDeclarationText",
			method:   f.formatDeclarationText,
			input:    "qubit[2]q",
			expected: "qubit[2] q",
		},
		{
			name:     "formatGateCallText simple",
			method:   f.formatGateCallText,
			input:    "hq[0]",
			expected: "h q[0]",
		},
		{
			name:     "formatGateCallText two-qubit",
			method:   f.formatGateCallText,
			input:    "cxq[0],q[1]",
			expected: "cx q[0], q[1]",
		},
		{
			name:     "formatMeasureText",
			method:   f.formatMeasureText,
			input:    "measureq->c",
			expected: "measure q -> c",
		},
		{
			name:     "formatAssignmentText",
			method:   f.formatAssignmentText,
			input:    "a=b+c",
			expected: "a = b+c",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := tt.method(tt.input)
			if result != tt.expected {
				t.Errorf("%s = %q, want %q", tt.name, result, tt.expected)
			}
		})
	}
}
