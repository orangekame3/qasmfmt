# GitHub Copilot Code Review Setup

This repository uses GitHub Copilot's official code review functionality.

## Prerequisites

One of the following GitHub Copilot plans is required:
- **Copilot Pro**: For individual use
- **Copilot Business**: For organizations
- **Copilot Enterprise**: For enterprise use

## Automated Setup

### Automatic Review Request via Workflow

Automatic review requests to GitHub Copilot are triggered when PRs are created:

- Automatically runs on PR creation, updates, and reopening
- Does not run on draft PRs
- Uses only `GITHUB_TOKEN` (no additional API keys required)

### Custom Instructions (Optional)

Review instructions can be customized in `.github/copilot-instructions.md`:
- Go language specific items
- Security checks
- QASM syntax validation
- Review language (English)

## Manual Review Request

1. Open the GitHub PR page
2. Select "Copilot" in the "Reviewers" section on the right
3. Review typically starts within 30 seconds

## Review Results

- Copilot provides "Comment" reviews only (does not approve)
- Suggested changes can be applied with a few clicks
- Thumbs up/down feedback available

## Important Notes

⚠️ **Important**: Always validate Copilot's feedback and supplement with human review.

## Monthly Quota

Each plan has a monthly review limit:
- Wait until next month if quota is reached
- Enterprise plans can configure additional quota

## Troubleshooting

### Copilot not appearing in reviewers
- Verify Copilot is enabled in the organization
- Confirm appropriate plan subscription

### Review not executing
- PR might be too large (consider splitting)
- Monthly quota might be reached