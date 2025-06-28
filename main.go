package main

import (
	"context"
	"os"

	"github.com/charmbracelet/fang"
	"github.com/orangekame3/qasmfmt/cmd"
	"github.com/spf13/cobra"
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "qasmfmt [file...]",
		Short: "A formatter for OpenQASM 3 files",
		Long: `qasmfmt is a command-line tool for formatting OpenQASM 3 quantum circuit files.

It automatically formats OpenQASM code with proper indentation, spacing, and structure
according to OpenQASM 3.0 specification.

Examples:
  qasmfmt example.qasm               # Format and print to stdout
  qasmfmt -w example.qasm            # Format in-place
  qasmfmt -c example.qasm            # Check if file is formatted
  qasmfmt -i 4 example.qasm          # Use 4 spaces for indentation
  qasmfmt *.qasm                     # Format multiple files`,
		Args: cobra.MinimumNArgs(1),
		RunE: cmd.RunRootFormat,
	}

	// Global flags
	rootCmd.PersistentFlags().BoolP("write", "w", false, "write result to (source) file instead of stdout")
	rootCmd.PersistentFlags().BoolP("check", "c", false, "check if the file(s) are formatted")
	rootCmd.PersistentFlags().UintP("indent", "i", 2, "number of spaces to use for indentation")
	rootCmd.PersistentFlags().BoolP("newline", "n", true, "end the file with a trailing newline")
	rootCmd.PersistentFlags().BoolP("verbose", "v", false, "verbose output")
	rootCmd.PersistentFlags().BoolP("diff", "d", false, "display diffs instead of rewriting files")

	// Add subcommands for backwards compatibility
	rootCmd.AddCommand(cmd.NewFormatCmd())
	rootCmd.AddCommand(cmd.NewCheckCmd())
	rootCmd.AddCommand(cmd.NewVersionCmd())
	rootCmd.AddCommand(cmd.NewCompletionCmd())

	if err := fang.Execute(context.Background(), rootCmd); err != nil {
		os.Exit(1)
	}
}
