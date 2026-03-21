---
title: Create A GitHub Repository From A Local Folder
slug: /cli-tools/github/create-repo-from-local/
summary: Quick GitHub CLI workflow for authenticating, creating a repository from the current folder, and pushing the initial commit.
topic: cli-tools/github
type: recipe
tags: [github, gh, git, repository]
aliases: [creating a new github repo from the terminal]
platforms: [windows, linux, macos]
related:
  - /cli-tools/winget/package-management-reference/
status: published
updated: 2026-03-20
---

## Goal

Create a new GitHub repository directly from an existing local project without leaving the terminal.

## Prerequisites

- `git` installed and configured
- GitHub CLI authenticated with `gh auth login`
- A local folder ready for its first commit

## Steps

1. Authenticate with GitHub CLI if this machine has not been set up yet.
2. Initialize Git in the project folder if needed.
3. Create the first commit.
4. Run `gh repo create` with `--source` and `--push`.

## Commands

```bash
gh auth login
cd /path/to/your/folder
git init
git add .
git commit -m "Initial commit"
gh repo create your-repo-name --source=. --push --private
```

## Verification

- Confirm the remote repository exists on GitHub.
- Run `git remote -v` and verify that `origin` points to the new repo.
- Check that the initial commit is visible in the repository history.

## Related

- [`winget Package Management Reference`](/cli-tools/winget/package-management-reference/)
