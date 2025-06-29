package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
	Version   = "0.0.11"
	Commit    = "unknown"
	BuildDate = "unknown"
)

func NewVersionCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "version",
		Short: "Print the version number of qasmfmt",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("qasmfmt version %s", Version)
			if Commit != "" {
				fmt.Printf(" (%s)", Commit)
			}
			if BuildDate != "" {
				fmt.Printf(" built on %s", BuildDate)
			}
			fmt.Println()
		},
	}
}
