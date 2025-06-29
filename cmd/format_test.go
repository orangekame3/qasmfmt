package cmd

import (
	"bytes"
	"strings"
	"testing"
)

func TestRunFormat_Stdin(t *testing.T) {
	tests := []struct {
		name           string
		input          string
		expectedOutput string
		expectedError  string
		wantErr        bool
	}{
		{
			name:           "valid QASM input",
			input:          "OPENQASM 3.0;qubit[2]q;hq[0];",
			expectedOutput: "OPENQASM 3.0;\nqubit[2] q;\nh q[0];\n",
			wantErr:        false,
		},
		{
			name:           "simple text input",
			input:          "malformed qasm",
			expectedOutput: "",
			wantErr:        false,
		},
		{
			name:           "empty input",
			input:          "",
			expectedOutput: "",
			wantErr:        false,
		},
		{
			name:           "complex QASM input",
			input:          "OPENQASM 3.0;include\"stdgates.qasm\";qubit[2]q;bit[2]c;hq[0];cxq[0],q[1];measureq->c;",
			expectedOutput: "OPENQASM 3.0;\ninclude \"stdgates.qasm\";\n\nqubit[2] q;\nbit[2] c;\nh q[0];\ncx q[0], q[1];\nmeasure q -> c;\n",
			wantErr:        false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cmd := NewFormatCmd()
			stdin := bytes.NewBufferString(tt.input)
			stdout := new(bytes.Buffer)
			stderr := new(bytes.Buffer)

			cmd.SetIn(stdin)
			cmd.SetOut(stdout)
			cmd.SetErr(stderr)

			err := cmd.RunE(cmd, []string{})

			if tt.wantErr {
				if err == nil {
					t.Fatalf("cmd.RunE() expected an error, got nil")
				}
				if !strings.Contains(err.Error(), tt.expectedError) {
					t.Errorf("Expected error message to contain %q, got %q", tt.expectedError, err.Error())
				}
				if stderr.String() == "" {
					t.Errorf("Expected stderr output, got empty")
				}
			} else {
				if err != nil {
					t.Fatalf("cmd.RunE() error = %v", err)
				}
				if stdout.String() != tt.expectedOutput {
					t.Errorf("Expected output %q, got %q", tt.expectedOutput, stdout.String())
				}
				if stderr.String() != "" {
					t.Errorf("Expected empty stderr, got %q", stderr.String())
				}
			}
		})
	}
}

func TestRunFormat_Files(t *testing.T) {
	tests := []struct {
		name     string
		args     []string
		wantErr  bool
		errorMsg string
	}{
		{
			name:    "no arguments - should use stdin",
			args:    []string{},
			wantErr: false,
		},
		{
			name:    "single file argument",
			args:    []string{"../testdata/test_simple.qasm"},
			wantErr: false,
		},
		{
			name:    "multiple file arguments",
			args:    []string{"../testdata/test_simple.qasm", "../testdata/test_gates.qasm"},
			wantErr: false,
		},
		{
			name:     "non-existent file",
			args:     []string{"nonexistent.qasm"},
			wantErr:  true,
			errorMsg: "failed to format",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cmd := NewFormatCmd()
			stdout := new(bytes.Buffer)
			stderr := new(bytes.Buffer)

			if len(tt.args) == 0 {
				stdin := bytes.NewBufferString("OPENQASM 3.0;\nqubit q;")
				cmd.SetIn(stdin)
			}

			cmd.SetOut(stdout)
			cmd.SetErr(stderr)

			err := cmd.RunE(cmd, tt.args)

			if tt.wantErr {
				if err == nil {
					t.Fatalf("cmd.RunE() expected an error, got nil")
				}
				if tt.errorMsg != "" && !strings.Contains(err.Error(), tt.errorMsg) {
					t.Errorf("Expected error message to contain %q, got %q", tt.errorMsg, err.Error())
				}
			} else {
				if err != nil {
					t.Fatalf("cmd.RunE() error = %v", err)
				}
			}
		})
	}
}
