package cmd

import (
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/spf13/cobra"
)

var (
	writeFlag   bool
	diffFlag    bool
	verboseFlag bool
)

func NewFormatCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "format [file...]",
		Short: "Format OpenQASM 3 files",
		Long:  `Format OpenQASM 3 files with proper indentation and style`,
		Args:  cobra.ArbitraryArgs,
		RunE:  runFormat,
	}

	cmd.Flags().BoolVarP(&writeFlag, "write", "w", false, "write result to (source) file instead of stdout")
	cmd.Flags().BoolVarP(&diffFlag, "diff", "d", false, "display diffs instead of rewriting files")
	cmd.Flags().BoolVarP(&verboseFlag, "verbose", "v", false, "verbose output")

	return cmd
}

func runFormat(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		// Read from stdin
		input, err := io.ReadAll(cmd.InOrStdin())
		if err != nil {
			return fmt.Errorf("failed to read from stdin: %w", err)
		}
		formatted, err := FormatQASM(string(input))
		if err != nil {
			fmt.Fprintf(cmd.ErrOrStderr(), "Error formatting from stdin: %v\n", err)
			return fmt.Errorf("failed to format QASM from stdin: %w", err)
		}
		fmt.Fprint(cmd.OutOrStdout(), formatted)
		return nil
	}

	var processedFiles int
	var modifiedFiles int

	for _, file := range args {
		modified, err := formatFile(file)
		if err != nil {
			if verboseFlag {
				fmt.Printf("❌ %s: %v\n", file, err)
			}
			return fmt.Errorf("failed to format %s: %w", file, err)
		}

		processedFiles++
		if modified {
			modifiedFiles++
			if verboseFlag && writeFlag {
				fmt.Printf("✅ %s formatted and saved\n", file)
			}
		} else if verboseFlag {
			fmt.Printf("ℹ️  %s already formatted\n", file)
		}
	}

	if verboseFlag && writeFlag {
		fmt.Printf("\n📊 Processed %d files, modified %d files\n", processedFiles, modifiedFiles)
	}

	return nil
}

func formatFile(filename string) (bool, error) {
	content, err := os.ReadFile(filename)
	if err != nil {
		return false, fmt.Errorf("failed to read file: %w", err)
	}

	formatted, err := FormatQASM(string(content))
	if err != nil {
		return false, fmt.Errorf("failed to format QASM: %w", err)
	}

	modified := string(content) != formatted

	if writeFlag {
		if modified {
			err := os.WriteFile(filename, []byte(formatted), 0600)
			return modified, err
		}
		return false, nil
	}

	if diffFlag {
		err := showDiff(filename, string(content), formatted)
		return modified, err
	}

	fmt.Print(formatted)
	return modified, nil
}

func showDiff(filename, original, formatted string) error {
	if original == formatted {
		return nil
	}

	fmt.Printf("--- %s\n", filename)
	fmt.Printf("+++ %s (formatted)\n", filename)

	originalLines := strings.Split(original, "\n")
	formattedLines := strings.Split(formatted, "\n")

	maxLines := len(originalLines)
	if len(formattedLines) > maxLines {
		maxLines = len(formattedLines)
	}

	for i := 0; i < maxLines; i++ {
		var origLine, formLine string

		if i < len(originalLines) {
			origLine = originalLines[i]
		}
		if i < len(formattedLines) {
			formLine = formattedLines[i]
		}

		if origLine != formLine {
			if origLine != "" {
				fmt.Printf("-%s\n", origLine)
			}
			if formLine != "" {
				fmt.Printf("+%s\n", formLine)
			}
		}
	}

	return nil
}