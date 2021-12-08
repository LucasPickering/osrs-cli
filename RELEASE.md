# Release Process

The release process for this tool is as follows:

- Cut a `release-x.y.z` branch
  - Update the version number in `Cargo.toml` to `x.y.z`
  - Commit with the message `x.y.z`
  - This branch should include **no** other changes, all patches should've already been merged to `master`
- Post and merge a PR for this branch
- Pull latest master (with the version update)
- Create a tag `x.y.z` locally and push it
  - This should kick off release CI, which will generate a draft release in GitHub
- Wait for release CI to pass and all artifacts to be attached to the draft release
- Update the draft release with patch notes
- Run `cargo publish`
- Publish the release
