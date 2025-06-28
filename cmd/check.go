package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func NewCheckCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "check [file...]",
		Short: "Check if OpenQASM 3 files are formatted",
		Long:  "Check if OpenQASM 3 files are properly formatted without modifying them",
		Args:  cobra.MinimumNArgs(1),
		RunE:  runCheck,
	}

	return cmd
}

func runCheck(cmd *cobra.Command, args []string) error {
	var hasUnformatted bool
	var checkedFiles int

	for _, file := range args {
		formatted, err := checkFile(file)
		if err != nil {
			fmt.Printf("❌ %s: %v\n", file, err)
			hasUnformatted = true
			continue
		}

		checkedFiles++
		if !formatted {
			hasUnformatted = true
			fmt.Printf("❌ %s is not formatted\n", file)
		} else {
			fmt.Printf("✅ %s is formatted correctly\n", file)
		}
	}

	if checkedFiles > 0 {
		if hasUnformatted {
			fmt.Printf("\n❌ Some files are not properly formatted\n")
			os.Exit(1)
		} else {
			fmt.Printf("\n✅ All %d files are formatted correctly\n", checkedFiles)
		}
	}

	return nil
}

func checkFile(filename string) (bool, error) {
	content, err := os.ReadFile(filename)
	if err != nil {
		return false, fmt.Errorf("failed to read file: %w", err)
	}

	formatted, err := FormatQASM(string(content))
	if err != nil {
		return false, fmt.Errorf("failed to format QASM: %w", err)
	}

	return string(content) == formatted, nil
}
