# Release Checklist

This is a list of steps to complete when making a new release.

1. Review the commit and PR history since last release. Ensure that all relevant
changes are included in `CHANGELOG.md`, and that breaking changes
are specifically annotated
1. Update the version of seed dependency in the readme.
1. Ensure both the readme and homepage website reflect API changes. Instructions
for updating the homepage are available [here](https://github.com/seed-rs/seed-homepage)
1. Update the homepage with the new release version (main page), and changelog
1. Ensure the [quickstart repo](https://github.com/seed-rs/seed-quickstart) is updated
to reflect API changes, and the new version
1. Ensure the version listed in `Cargo.toml` is updated
1. Update Rust tools: `rustup update`
1. Run `cargo make populate_all` to synchronize `St`, `At` and other enums with official values
1. Run `cargo make verify` to ensure tests pass, and `clippy` / `fmt` are run
1. Commit and push the repo
1. Check that CI pipeline passed
1. Run `cargo package`
1. Run `cargo publish`
1. Add a release on [Github](https://github.com/seed-rs/seed/releases), following the format of previous releases
1. Verify the [docs page](https://docs.rs/seed/) updated correctly
1. Clone the quickstart repo, and verify it builds and runs correctly

