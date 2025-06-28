package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func NewCompletionCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "completion [bash|zsh|fish|powershell]",
		Short: "Generate the autocompletion script for the specified shell",
		Long: `To load completions:

Bash:

  $ source <(qasmfmt completion bash)

  # To load completions for each session, execute once:
  # Linux:
  $ qasmfmt completion bash > /etc/bash_completion.d/qasmfmt
  # macOS:
  $ qasmfmt completion bash > /usr/local/etc/bash_completion.d/qasmfmt

Zsh:

  # If shell completion is not already enabled in your environment,
  # you will need to enable it.  You can execute the following once:

  $ echo "autoload -U compinit; compinit" >> ~/.zshrc

  # To load completions for each session, execute once:
  $ qasmfmt completion zsh > "${fpath[1]}/_qasmfmt"

  # You will need to start a new shell for this setup to take effect.

fish:

  $ qasmfmt completion fish | source

  # To load completions for each session, execute once:
  $ qasmfmt completion fish > ~/.config/fish/completions/qasmfmt.fish

PowerShell:

  PS> qasmfmt completion powershell | Out-String | Invoke-Expression

  # To load completions for every new session, run:
  PS> qasmfmt completion powershell > qasmfmt.ps1
  # and source this file from your PowerShell profile.
`,
		DisableFlagsInUseLine: true,
		ValidArgs:             []string{"bash", "zsh", "fish", "powershell"},
		Args:                  cobra.MatchAll(cobra.ExactArgs(1), cobra.OnlyValidArgs),
		Run: func(cmd *cobra.Command, args []string) {
			var err error
			switch args[0] {
			case "bash":
				err = cmd.Root().GenBashCompletion(os.Stdout)
			case "zsh":
				err = cmd.Root().GenZshCompletion(os.Stdout)
			case "fish":
				err = cmd.Root().GenFishCompletion(os.Stdout, true)
			case "powershell":
				err = cmd.Root().GenPowerShellCompletionWithDesc(os.Stdout)
			}
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error generating completion: %v\n", err)
				os.Exit(1)
			}
		},
	}

	return cmd
}
