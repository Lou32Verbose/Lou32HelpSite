# Lou32HelpSite

This repository hosts https://lou32verbose.github.io/Lou32HelpSite/ via GitHub Pages.

It does **not** contain source code. The site is built from
[Lou32Verbose/Lou32HelpModuleDocumentLibrary](https://github.com/Lou32Verbose/Lou32HelpModuleDocumentLibrary)
by `.github/workflows/pages.yml`, which checks that repo out at `master`,
runs `cargo run -- build`, and deploys the resulting `dist/site` artifact.

## Editing the site

Make changes in the source repo, not here. Pushes to `master` on the source
repo automatically trigger a redeploy via a `repository_dispatch` event.

To trigger a manual redeploy without a source change, use the **Actions** tab:
run **Deploy to GitHub Pages** with the `workflow_dispatch` button.
