package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

// Config holds the formatting configuration
type Config struct {
	Write   bool
	Check   bool
	Indent  uint
	Newline bool
	Verbose bool
	Diff    bool
}

func RunRootFormat(cmd *cobra.Command, args []string) error {
	config, err := getConfigFromFlags(cmd)
	if err != nil {
		return err
	}

	if config.Check {
		return runCheckWithConfig(cmd, args, config)
	}

	return runFormatWithConfig(cmd, args, config)
}

func getConfigFromFlags(cmd *cobra.Command) (*Config, error) {
	write, err := cmd.Flags().GetBool("write")
	if err != nil {
		return nil, err
	}

	check, err := cmd.Flags().GetBool("check")
	if err != nil {
		return nil, err
	}

	indent, err := cmd.Flags().GetUint("indent")
	if err != nil {
		return nil, err
	}

	newline, err := cmd.Flags().GetBool("newline")
	if err != nil {
		return nil, err
	}

	verbose, err := cmd.Flags().GetBool("verbose")
	if err != nil {
		return nil, err
	}

	diff, err := cmd.Flags().GetBool("diff")
	if err != nil {
		return nil, err
	}

	return &Config{
		Write:   write,
		Check:   check,
		Indent:  indent,
		Newline: newline,
		Verbose: verbose,
		Diff:    diff,
	}, nil
}

func runFormatWithConfig(cmd *cobra.Command, args []string, config *Config) error {
	var processedFiles int
	var modifiedFiles int

	for _, file := range args {
		modified, err := formatFileWithConfig(file, config)
		if err != nil {
			if config.Verbose {
				fmt.Printf("❌ %s: %v\n", file, err)
			}
			return fmt.Errorf("failed to format %s: %w", file, err)
		}

		processedFiles++
		if modified {
			modifiedFiles++
			if config.Verbose && config.Write {
				fmt.Printf("✅ %s formatted and saved\n", file)
			}
		} else if config.Verbose {
			fmt.Printf("ℹ️  %s already formatted\n", file)
		}
	}

	if config.Verbose && config.Write {
		fmt.Printf("\n📊 Processed %d files, modified %d files\n", processedFiles, modifiedFiles)
	}

	return nil
}

func runCheckWithConfig(cmd *cobra.Command, args []string, config *Config) error {
	var hasUnformatted bool
	var checkedFiles int

	for _, file := range args {
		formatted, err := checkFileWithConfig(file, config)
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

func formatFileWithConfig(filename string, config *Config) (bool, error) {
	content, err := os.ReadFile(filename)
	if err != nil {
		return false, fmt.Errorf("failed to read file: %w", err)
	}

	formatted, err := FormatQASMWithConfig(string(content), config)
	if err != nil {
		return false, fmt.Errorf("failed to format QASM: %w", err)
	}

	modified := string(content) != formatted

	if config.Write {
		if modified {
			err := os.WriteFile(filename, []byte(formatted), 0600)
			return modified, err
		}
		return false, nil
	}

	if config.Diff {
		err := showDiff(filename, string(content), formatted)
		return modified, err
	}

	fmt.Print(formatted)
	return modified, nil
}

func checkFileWithConfig(filename string, config *Config) (bool, error) {
	content, err := os.ReadFile(filename)
	if err != nil {
		return false, fmt.Errorf("failed to read file: %w", err)
	}

	formatted, err := FormatQASMWithConfig(string(content), config)
	if err != nil {
		return false, fmt.Errorf("failed to format QASM: %w", err)
	}

	return string(content) == formatted, nil
}
