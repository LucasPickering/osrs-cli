# Release Process

## Full CLI release

The release process for this tool is as follows:

- Cut a `release-x.y.z` branch
  - Update the version number in `Cargo.toml` to `x.y.z`
  - Commit with the message `x.y.z`
  - This branch should include **no** other changes, all patches should've already been merged to `master`
- Post and merge a PR for this branch into `master`
- Merge all changes since the last release into the `release`
  - Note: this _can't_ be done via PR, because we need a fast-forward merge with all the same commit hashes as `master`, which [isn't possible on GitHub](https://stackoverflow.com/questions/60597400/how-to-do-a-fast-forward-merge-on-github)
  - ```
    git checkout master
    git pull
    git checkout release
    git pull
    git merge --ff master
    git push
    ```
- Wait for the release CI jobs to run
  - This will create a GitHub Release, push the appropriate git tag, and publish the latest package to crates.io, and attach all artifacts to the draft release
- Update the draft release with patch notes
- Publish the release

## Website-only release
