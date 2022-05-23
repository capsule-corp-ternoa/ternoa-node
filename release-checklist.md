# Runtime Releases
Copy this workflow into your pull request.

- [ ] Create a release branch
- [ ] Verify spec_version has been increase since the last release
- [ ] Verify previous completed migrations are removed
- [ ] Verify pallet and extrinsic ordering have stayed the same. Bump transaction_version if not
- [ ] Verify benchmarks/weights have been updated/added for any modified or new runtime logic.
- [ ] Verify that the upgrade won't brick the chain
- [ ] Verify that the storage migration has been done correctly using test data
- [ ] Verify that Product QA has been done
- [ ] Check that the build artifacts have been built
  - [ ] Linux Client binary
  - [ ] WASM binary file
- [ ] Check that a github draft release has been created with relevant release notes
- [ ] Check that all items listed in the milestone are included in the release
- [ ] Check that the github draft has been convert to a full release