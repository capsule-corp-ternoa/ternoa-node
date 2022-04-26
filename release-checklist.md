# Runtime Releases
These checks should be performed on the codebase prior on creating an release candidate or full release tag.
- [ ] Verify `spec_version` has been increase since the last release.
- [ ] Verify previous completed migrations are removed.
- [ ] Verify pallet and extrinsic ordering has stayed the same. Bump `transaction_version` if not.
- [ ] Verify benchmarks have been updated for any modified or new runtime logic.
- [ ] Verify that the upgrade won't brick the chain.
- [ ] Verify that the storage migration has been done correctly using test data.
- [ ] Verify that Product QA has been done.

These checks should be performed once a release candidate or full release tag has been created.
- [ ] Check that the build artifacts have been built.
    - [ ] Linux Client binary
    - [ ] WASM binary file
    - [ ] Source code
- [ ] Check that a github draft release has been created with relevant release notes. [LINK]()
- [ ] Check that all items listed in the milestone are included in the release. [LINK]()

These checks should be done after all the checks have been done.
- [ ] Check that the github draft has been convert to a full release.