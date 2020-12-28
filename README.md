# chain
Ternoa's Blockchain to support the secure creation and transfer of Capsules.

## Notes
- Fees and tips are 100% awarded to block authors
- Staking and block rewards are created from the void, thus creating more coins
- Staking slashes and remainders are burned, you may want to send them to a treasury later
- There are no collectives, treasuries or democracy pallets
- We kept the runtime slim and did not include less core pallets such as `recovery`, `utility` or `vesting`
- In local and dev chains we have configured `Alice` to have access to the `sudo` pallet which we added to test runtime migrations and upgrades

### Capsules
- IDs are represented with a u32, we have a maximum number of capsules of u32 MAX
- For now we do not charge for capsule creation, we may want to change this in the future

### Timed Escrow
- capsule locking

### Testing
- test --all for tests
- test --all --all-features for benches, cannot be individual

### Weights / Benchmarking
- done on Apple M1
- may need to recomputing when defining the validator grade hardware (provide instructions)