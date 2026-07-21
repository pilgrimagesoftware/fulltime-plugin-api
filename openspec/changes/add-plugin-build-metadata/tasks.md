## 1. Manifest Schema

- [ ] 1.1 Add `developer: Option<String>` and `build_date: Option<String>` to `Manifest` in
  `src/manifest.rs`, each with a doc comment noting `build_date` is conventionally RFC 3339 but
  unvalidated (matching `Fixture.kickoff`'s treatment)
- [ ] 1.2 Add the corresponding optional fields to `RawManifest`
- [ ] 1.3 Add `ManifestField::Developer` and `ManifestField::BuildDate` variants, including their
  `Display` impl arm

## 2. Parsing and Validation

- [ ] 2.1 In `Manifest::parse`, thread both new fields through as `Option<String>`, defaulting to
  `None` when absent
- [ ] 2.2 Reject a present-but-empty/whitespace-only `developer` or `build_date` with
  `ManifestError::InvalidField`, reusing (or extracting into a shared helper alongside)
  `network_hosts`'s existing empty-entry check

## 3. Tests

- [ ] 3.1 Unit test: manifest omitting both fields parses successfully with both `None`
- [ ] 3.2 Unit test: manifest declaring both fields parses successfully and exposes them
  unchanged
- [ ] 3.3 Unit test: empty `developer` field is rejected with
  `ManifestField::Developer`
- [ ] 3.4 Unit test: empty `build_date` field is rejected with `ManifestField::BuildDate`
- [ ] 3.5 Update the crate-level doc example in `src/lib.rs` and/or `README.md` if either shows a
  full manifest, so they stay accurate (additive fields, no required change, but worth checking)

## 4. Release

- [ ] 4.1 Update `CHANGELOG.md`'s `[Unreleased]` section describing the additive manifest change
- [ ] 4.2 Confirm `RELEASING.md`'s process results in a minor version bump (additive manifest
  field), not a patch or major
