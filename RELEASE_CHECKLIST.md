# Release Checklist

This is a list of steps to complete when making a new release.

# Before the release

- [ ] 1. Create a new issue in the Seed repo with the name `Seed x.x.x` and copy-paste this checklist into it (also add blockers and additional tasks, if exist).
- [ ] 2. Update all official examples.
- [ ] 3. Review the commit and PR history since last release. Ensure that all relevant
changes are included in `CHANGELOG.md`, and that breaking changes
are specifically annotated.
- [ ] 4. Ensure the `README.md` reflects API changes.
- [ ] 5. Update the `CHANGELOG.md` with the new release version.
- [ ] 6. Ensure the version listed in `Cargo.toml` is updated.
- [ ] 7. Update Rust tools: `rustup update`.
- [ ] 8. Run `cargo make populate_all` to synchronize `St`, `At` and other enums with official values.
- [ ] 9. Run `cargo make verify` to ensure tests pass, and `clippy` / `fmt` are run.
- [ ] 10. Commit and push the repo.
- [ ] 11. Check that CI pipeline passed.
- [ ] 12. Run `cargo package`.
- [ ] 13. Run `cargo publish`.
- [ ] 14. Add a release on [Github](https://github.com/seed-rs/seed/releases), following the format of previous releases.
- [ ] 15. Verify the [docs page](https://docs.rs/seed/) updated correctly.

# After the release

- [ ] 16. Update all quickstarts.
- [ ] 17. Write documentation for the current release on the website.
- [ ] 18. Make sure the website's version selector shows the released version by default. 
- [ ] 19. Notify authors of community tutorials, quickstarts and examples about a new Seed version.
- [ ] 20. Write announcements (chat, forum, etc.).

