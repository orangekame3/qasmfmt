package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
	version   = "dev"
	commit    = "unknown"
	buildDate = "unknown"
)

func NewVersionCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "version",
		Short: "Print the version number of qasmfmt",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("qasmfmt version %s\n", getVersionInfo())
		},
	}

	return cmd
}

func getVersionInfo() string {
	info := version
	if commit != "unknown" {
		info += " (" + commit + ")"
	}
	if buildDate != "unknown" {
		info += " built on " + buildDate
	}
	return info
}
