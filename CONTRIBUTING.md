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
  - [Documentation](#documentation)
  - [Formatting](#formatting)
  - [Benchmarks](#benchmarks)
  - [Tests](#tests)
  - [Runtime storage | Migration](#runtime-storage--migration)
  - [Types and Metadata](#types-and-metadata)
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
Branch names should use the following convention: `author-name/issue-that-is-begin-fixed`

Example: `alex/add-minting-fee-to-capsules`

## Documentation
Any piece of code that will be used by a third party needs to be well documented and this includes but is not limited to the following:
- Dispatchable functions
- Trait methods
- Events
- Errors
- Storage data

## Formatting
Before creating a Pull Request, the code needs to be formatted using the command `cargo fmt --all`. 

## Benchmarks
Benchmarks needs to be written for every new dispatchable function that is added. They are used to calculate weights which are being used to calculate fees for those extrinsics. 

Inside every pallet there is a `default_weights.rs` file which contains the weights and the command which was executed in order to get those weights.

## Tests
Tests are used to prove that the code is correct and to convey system usage and constraints. Every dispatchable function, trait method or migration function should have at least one test associated with it.

## Runtime storage | Migration
In case of changing how the runtime storage is ordered for existing objects, either by adding new properties or changing existing properties, it needs to be handled by writing a functions which will allow nodes with older versions of storage to safely and gracefully upgrade to a newer version.

Example code: [nfts pallet](pallets/nfts/src/migration.rs).

## Types and Metadata
User defined structures are not by default recognized by the PolkadotJs UI. In order to be recognized a JSON file, which contains the description of all types, needs to be supplied to the webapp.

This means that if user defined structures are changed or a new structure is added, it needs to be also reflected in the [types.json](types.json) file. 

# Workflow

- [ ] Create a branch
- [ ] Implement feature/bugfix
- [ ] If a new dispatchable function was added:
  - [ ] Write tests
  - [ ] Write benchmarks
  - [ ] Generate weights
- [ ] If existing dispatchable code was changed:
  - [ ] Update tests
  - [ ] Generate new weights
- [ ] If runtime storage has changed:
  - [ ] Bump pallet version
  - [ ] Write migration code
  - [ ] Write tests for the migration code
  - [ ] Manually Test migration
- [ ] If a user defined structure is added/changed:
  - [ ] Update `types.json`
- [ ] If trait code was changed/added
  - [ ] Update/Write tests
- [ ] Run `cargo fmt`
- [ ] Manual testing
- [ ] Bump spec or impl version