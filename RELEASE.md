# Release Process

## Full CLI release

- Run `cargo release <major|minor|patch>`
  - If it looks good, add `--execute`

This will kick off CI to build and publish the releases.

## Website-only release

To release changes to the website without publishing a new version of the CLI, just run [the merge workflow](https://github.com/LucasPickering/osrs-cli/actions/workflows/merge-release.yml).
