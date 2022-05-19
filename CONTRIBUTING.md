<h1 align="center">
    Contributing to Ternoa Blockchain 🚀
</h1>

Thank you for your interest in contributing to Ternoa Blockchain!

Table of Contents:

- [Feedback](#feedback)
  - [Feature Requests](#feature-requests)
  - [Bug Reports](#bug-reports)
  - [General discussion](#general-discussion)
  - [Chat](#chat)
- [Code Contributions](#code-contributions)
  - [Branch names](#branch-names)
  - [Workflow](#workflow)

</br>

# Feedback

## Feature Requests
Feature requests should be reported in the
[Ternoa issue tracker](https://github.com/capsule-corp-ternoa/chain/issues). 

## Bug Reports
Bug reports should be reported in the
[Ternoa issue tracker](https://github.com/capsule-corp-ternoa/chain/issues).

## General discussion
General discussion and remarks should be conveyed in the [Ternoa discussions tracker](https://github.com/capsule-corp-ternoa/chain/discussions)

## Chat
The quickest and most open way to communicate with the Ternoa Blockchain team is on our [discord server]("https://discord.gg/cNZTGtGJNR"). 

# Code Contributions

## Branch names
Branch names should use the following convention: `release/version-number`

Example:  `release/1.0.1-rc1` or `release/1.1.0`

## Workflow
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