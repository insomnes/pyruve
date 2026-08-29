# Releasing pyruve

The release workflow publishes a tag through crates.io Trusted Publishing.
It does not use a long-lived `CARGO_REGISTRY_TOKEN` secret.

## One-time setup

1. Push `.github/workflows/release.yml` to the default branch.
2. Create a GitHub environment named `release` in the repository settings.
3. Open the `pyruve` crate settings on crates.io.
4. Add a GitHub Actions trusted publisher with these values:

   - GitHub owner: `insomnes`
   - Repository: `pyruve`
   - Workflow filename: `release.yml`
   - Environment: `release`

Add environment protection rules in GitHub if releases must require manual approval.

## Release procedure

1. Update `Cargo.toml` and `CHANGELOG.md`.
2. Run the local checks:

   ```console
   cargo fmt --all -- --check
   cargo test --locked
   cargo clippy --all-targets --locked -- -D warnings
   cargo publish --dry-run --locked
   ```

3. Push the release commits and wait for CI to pass on `main`.
4. Create and push an annotated version tag:

   ```console
   git tag -a v0.2.0 -m "pyruve 0.2.0"
   git push origin v0.2.0
   ```

5. Confirm that the `Publish crate` workflow completed successfully.
6. Confirm the new version on crates.io.

Published crate versions are immutable.
If publication succeeded but a later workflow step failed, do not reuse or replace the version.
