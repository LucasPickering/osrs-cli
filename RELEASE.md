# Release Process

## Full CLI release

The release process for this tool is as follows:

- Cut a `release-x.y.z` branch
  - Update the version number in `Cargo.toml` to `x.y.z`
  - Commit with the message `x.y.z`
  - This branch should include **no** other changes, all patches should've already been merged to `master`
- Post and merge a PR for this branch into `master`
- Run [the merge workflow](https://github.com/LucasPickering/osrs-cli/actions/workflows/merge-release.yml) from `master` to merge all changes into the `release` branch
  - This will also create a new git tag for the latest crate version, which will trigger a workflow to do the following:
    - Create a draft release on GitHub
    - Publish to crates.io
    - Attach build artifacts to the draft release
- Wait for the release CI jobs to finish
- Update the draft release with patch notes
- Publish the release

## Website-only release

To release changes to the website without publishing a new version of the CLI, just run [the merge workflow](https://github.com/LucasPickering/osrs-cli/actions/workflows/merge-release.yml).
