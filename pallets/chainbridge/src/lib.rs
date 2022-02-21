// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Pallet for bridging Polkadot Substrate and Ethereum chains.
//!
//! This pallet implement a general-purpose bridge to pass arbitrary messages
//! Polkadot Substrate Chain and Ethereum or any other target network.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//! This pallet is used for bridging chains.
//!
//! ## Terminology
//!
//! ## Usage
//!
//! ## Interface
//!
//! ### Supported Origins
//!
//! Signed origin is valid.
//!
//! ### Types
//! Event - Associated type for Event enum
//! AdminOrigin - Origin used for pallet administration
//! Proposal - Proposed dispatchable call
//! ChainId - The identifier for this chain (a constant). This must be unique and must not collide with existing IDs within a set of bridged chains.
//! PalletId - Identifier of the pallet (a constant).
//! ProposalLifetime -
//! RelayerVoteThreshold - Initial number of votes required for a proposal to be executed (see [RelayerVoteThreshold] in storage section).
//! WeightInfo- Weight information for extrinsics in this pallet.

//! ### Events
//! RelayerThresholdChanged(u32) - The relayer's vote threshold has been modified (new_threshold).
//! ChainWhitelisted(ChainId) - Chain now available for transfers (chain_id).
//! RelayerAdded(T::AccountId) - A new relayer was added to the brige.
//! RelayerRemoved(T::AccountId) - A relayer was removed from the bridge.
//! FungibleTransfer(ChainId, DepositNonce, ResourceId, U256, Vec<u8>) - A fungible asset was relayed.
//! VoteFor(ChainId, DepositNonce, T::AccountId) - Vote submitted in favour of proposal
//! VoteAgainst(ChainId, DepositNonce, T::AccountId) - Vote submitted against proposal
//! ProposalApproved(ChainId, DepositNonce) - Voting successful for a proposal
//! ProposalRejected(ChainId, DepositNonce) - Voting rejected a proposal
//! ProposalSucceeded(ChainId, DepositNonce) - Execution of call succeeded
//! ProposalFailed(ChainId, DepositNonce) - Execution of call failed
//!
//! ### Errors
//! `ThresholdNotSet - The relayer's vote threshold is not set
//! `InvalidThreshold` - The relayer's vote threshold cannot be zero
//! `InvalidChainId` - The provided chain Id is not valid
//! `ChainNotWhitelisted` - Interactions with this chain is not permitted
//! `ChainAlreadyWhitelisted - Chain has already been enabled
//! `ResourceDoesNotExist` - Resource ID provided isn't mapped to anything
//! `RelayerAlreadyExists` - Relayer already in set
//! `RelayerInvalid` - Provided accountId is not a relayer
//! `MustBeRelayer` - Protected operation, must be performed by relayer
//! `RelayerAlreadyVoted` - Relayer has already submitted some vote for this proposal
//! `ProposalAlreadyExists` - A proposal with these parameters has already been submitted
//! `ProposalDoesNotExist` - No proposal with the ID was found
//! `ProposalNotComplete` - Cannot complete proposal, needs more votes
//! `ProposalAlreadyComplete` - Proposal has either failed or succeeded
//! `ProposalExpired` - Lifetime of proposal has been exceeded
//!
//! ### Dispatchable Functions
//!
//! Callable functions (or extrinsics), also considered as transactions, materialize the
//! pallet contract. Here's the callable functions implemented in this module:
//!
//!
//! ### Public Functions
//!
//! ## Genesis Configuration
//! This pallet does not depends on the [`GenesisConfig`].
//!
//! ## Related Pallets
//! This pallet is tightly coupled to the following pallets:
//! - Centrifuge Chain [`bridge` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/bridge).
//! - Centrifuge Chain [`bridge_mapping` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/bridge-mapping).
//!
//! ## References
//! - [Substrate FRAME v2 attribute macros](https://crates.parity.io/frame_support/attr.pallet.html).
//!
//! ## Credits
//! The Centrifugians Tribe <tribe@centrifuge.io>
//!
//! ## License
//! GNU General Public License, Version 3, 29 June 2007 <https://www.gnu.org/licenses/gpl-3.0.html>

// Ensure we're `no_std` when compiling for WebAssembly.
#![cfg_attr(not(feature = "std"), no_std)]

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

// Declare crate mocking and testing modules
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// Declare crate component modules
pub mod constants;
mod traits;
pub mod types;
mod weights;

// Import Substrate modules
use codec::EncodeLike;

use frame_support::{
    dispatch::DispatchResult,
    ensure,
    traits::{EnsureOrigin, Get},
    weights::GetDispatchInfo,
    PalletId, Parameter,
};

use frame_system::{ensure_root, ensure_signed, RawOrigin as SystemOrigin};

use sp_core::U256;

use sp_runtime::traits::{AccountIdConversion, Dispatchable};

use sp_std::prelude::*;

// Re-export crate types and constants, and some traits
pub use constants::*;
pub use traits::WeightInfo;
pub use types::*;

// Re-export pallet components in crate namespace (for runtime construction)
pub use pallet::*;

// ----------------------------------------------------------------------------
// Pallet module
// ----------------------------------------------------------------------------

// Chain bridge pallet module
//
// The name of the pallet is provided by `construct_runtime` and is used as
// the unique identifier for the pallet's storage. It is not defined in the
// pallet itself.
#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // Bridge pallet type declaration.
    //
    // This structure is a placeholder for traits and functions implementation
    // for the pallet.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // ------------------------------------------------------------------------
    // Pallet configuration
    // ------------------------------------------------------------------------

    /// Chain bridge pallet's configuration trait.
    ///
    /// Associated types and constants are declared in this trait. If the pallet
    /// depends on other super-traits, the latter must be added to this config
    /// trait. Note that [`frame_system::Config`] must always be included.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Associated type for Event enum
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Origin used for pallet administration
        type AdminOrigin: EnsureOrigin<Self::Origin>;

        /// Proposed dispatchable call
        type Proposal: Parameter
            + Dispatchable<Origin = Self::Origin>
            + EncodeLike
            + GetDispatchInfo;

        /// The identifier for this chain.
        /// This must be unique and must not collide with existing IDs within a set of bridged chains.
        #[pallet::constant]
        type ChainId: Get<ChainId>;

        /// Constant configuration parameter to store the module identifier for the pallet.
        ///
        /// The module identifier may be of the form ```PalletId(*b"chnbrdge")``` and set
        /// using the [`parameter_types`](https://substrate.dev/docs/en/knowledgebase/runtime/macros#parameter_types)
        // macro in the [`runtime/lib.rs`] file.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        #[pallet::constant]
        type ProposalLifetime: Get<Self::BlockNumber>;

        /// Type for setting initial number of votes required for a proposal to be executed (see [RelayerVoteThreshold] in storage section).
        #[pallet::constant]
        type RelayerVoteThreshold: Get<u32>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    // ------------------------------------------------------------------------
    // Pallet events
    // ------------------------------------------------------------------------

    // The macro generates event metadata and derive Clone, Debug, Eq, PartialEq and Codec
    #[pallet::event]
    // The macro generates a function on Pallet to deposit an event
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Vote threshold has changed (new_threshold)
        RelayerThresholdChanged(u32),
        /// Chain now available for transfers (chain_id)
        ChainWhitelisted(ChainId),
        /// Relayer added to set
        RelayerAdded(T::AccountId),
        /// Relayer removed from set
        RelayerRemoved(T::AccountId),
        /// FunglibleTransfer is for relaying fungibles (dest_id, nonce, resource_id, amount, recipient, metadata)
        FungibleTransfer(ChainId, DepositNonce, ResourceId, U256, Vec<u8>),
        /// Vote submitted in favour of proposal
        VoteFor(ChainId, DepositNonce, T::AccountId),
        /// Vot submitted against proposal
        VoteAgainst(ChainId, DepositNonce, T::AccountId),
        /// Voting successful for a proposal
        ProposalApproved(ChainId, DepositNonce),
        /// Voting rejected a proposal
        ProposalRejected(ChainId, DepositNonce),
        /// Execution of call succeeded
        ProposalSucceeded(ChainId, DepositNonce),
        /// Execution of call failed
        ProposalFailed(ChainId, DepositNonce),
    }

    // ------------------------------------------------------------------------
    // Pallet storage items
    // ------------------------------------------------------------------------

    /// All whitelisted chains and their respective transaction counts
    #[pallet::storage]
    #[pallet::getter(fn get_chains)]
    pub(super) type ChainNonces<T: Config> =
        StorageMap<_, Blake2_256, ChainId, DepositNonce, OptionQuery>;

    /// Number of votes required for a proposal to execute
    #[pallet::storage]
    #[pallet::getter(fn get_threshold)]
    pub(super) type RelayerVoteThreshold<T: Config> =
        StorageValue<_, u32, ValueQuery, <T as Config>::RelayerVoteThreshold>;

    /// Tracks current relayer set
    #[pallet::storage]
    #[pallet::getter(fn get_relayers)]
    pub(super) type Relayers<T: Config> = StorageMap<_, Blake2_256, T::AccountId, bool, ValueQuery>;

    /// Number of relayers in set
    #[pallet::storage]
    #[pallet::getter(fn get_relayer_count)]
    pub(super) type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// All known proposals.
    /// The key is the hash of the call and the deposit ID, to ensure it's unique.
    #[pallet::storage]
    #[pallet::getter(fn get_votes)]
    pub(super) type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_256,
        ChainId,
        Blake2_256,
        (DepositNonce, T::Proposal),
        ProposalVotes<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;

    /// Utilized by the bridge software to map resource IDs to actual methods
    #[pallet::storage]
    #[pallet::getter(fn get_resources)]
    pub(super) type Resources<T: Config> =
        StorageMap<_, Blake2_256, ResourceId, Vec<u8>, OptionQuery>;

    // ------------------------------------------------------------------------
    // Pallet errors
    // ------------------------------------------------------------------------

    #[pallet::error]
    pub enum Error<T> {
        /// Relayer threshold not set
        ThresholdNotSet,
        /// Provided chain Id is not valid
        InvalidChainId,
        /// Relayer threshold cannot be 0
        InvalidThreshold,
        /// Interactions with this chain is not permitted
        ChainNotWhitelisted,
        /// Chain has already been enabled
        ChainAlreadyWhitelisted,
        /// Resource ID provided isn't mapped to anything
        ResourceDoesNotExist,
        /// Relayer already in set
        RelayerAlreadyExists,
        /// Provided accountId is not a relayer
        RelayerInvalid,
        /// Protected operation, must be performed by relayer
        MustBeRelayer,
        /// Relayer has already submitted some vote for this proposal
        RelayerAlreadyVoted,
        /// A proposal with these parameters has already been submitted
        ProposalAlreadyExists,
        /// No proposal with the ID was found
        ProposalDoesNotExist,
        /// Cannot complete proposal, needs more votes
        ProposalNotComplete,
        /// Proposal has either failed or succeeded
        ProposalAlreadyComplete,
        /// Lifetime of proposal has been exceeded
        ProposalExpired,
    }

    // ------------------------------------------------------------------------
    // Pallet dispatchable functions
    // ------------------------------------------------------------------------

    // Declare Call struct and implement dispatchable (or callable) functions.
    //
    // Dispatchable functions are transactions modifying the state of the chain. They
    // are also called extrinsics are constitute the pallet's public interface.
    // Note that each parameter used in functions must implement `Clone`, `Debug`,
    // `Eq`, `PartialEq` and `Codec` traits.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Sets the vote threshold for proposals.
        ///
        /// This threshold is used to determine how many votes are required
        /// before a proposal is executed.
        ///
        /// # <weight>
        /// - O(1) lookup and insert
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_threshold())]
        pub fn set_threshold(origin: OriginFor<T>, threshold: u32) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::set_relayer_threshold(threshold)
        }

        /// Stores a method name on chain under an associated resource ID.
        ///
        /// # <weight>
        /// - O(1) write
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_resource())]
        pub fn set_resource(
            origin: OriginFor<T>,
            id: ResourceId,
            method: Vec<u8>,
        ) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::register_resource(id, method)
        }

        /// Removes a resource ID from the resource mapping.
        ///
        /// After this call, bridge transfers with the associated resource ID will
        /// be rejected.
        ///
        /// # <weight>
        /// - O(1) removal
        /// # </weight>
        #[pallet::weight(<T as Config>::WeightInfo::remove_resource())]
        pub fn remove_resource(origin: OriginFor<T>, id: ResourceId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::unregister_resource(id)
        }

        /// Enables a chain ID as a source or destination for a bridge transfer.
        ///
        /// # <weight>
        /// - O(1) lookup and insert
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::whitelist_chain())]
        pub fn whitelist_chain(origin: OriginFor<T>, id: ChainId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::whitelist(id)
        }

        /// Adds a new relayer to the relayer set.
        ///
        /// # <weight>
        /// - O(1) lookup and insert
        /// # </weight>
        #[pallet::weight(<T as Config>::WeightInfo::add_relayer())]
        pub fn add_relayer(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::register_relayer(v)
        }

        /// Removes an existing relayer from the set.
        ///
        /// # <weight>
        /// - O(1) lookup and removal
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::remove_relayer())]
        pub fn remove_relayer(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::unregister_relayer(account_id)
        }

        /// Commits a vote in favour of the provided proposal.
        ///
        /// If a proposal with the given nonce and source chain ID does not already exist, it will
        /// be created with an initial vote in favour from the caller.
        ///
        /// # <weight>
        /// - weight of proposed call, regardless of whether execution is performed
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::acknowledge_proposal(call.get_dispatch_info().weight))]
        pub fn acknowledge_proposal(
            origin: OriginFor<T>,
            nonce: DepositNonce,
            src_id: ChainId,
            r_id: ResourceId,
            call: Box<<T as Config>::Proposal>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_relayer(&who), Error::<T>::MustBeRelayer);
            ensure!(
                Self::chain_whitelisted(src_id),
                Error::<T>::ChainNotWhitelisted
            );
            ensure!(
                Self::resource_exists(r_id),
                Error::<T>::ResourceDoesNotExist
            );

            Self::vote_for(who, nonce, src_id, call)
        }

        /// Commits a vote against a provided proposal.
        ///
        /// # <weight>
        /// - Fixed, since execution of proposal should not be included
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::reject_proposal())]
        pub fn reject_proposal(
            origin: OriginFor<T>,
            nonce: DepositNonce,
            src_id: ChainId,
            r_id: ResourceId,
            call: Box<<T as Config>::Proposal>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_relayer(&who), Error::<T>::MustBeRelayer);
            ensure!(
                Self::chain_whitelisted(src_id),
                Error::<T>::ChainNotWhitelisted
            );
            ensure!(
                Self::resource_exists(r_id),
                Error::<T>::ResourceDoesNotExist
            );

            Self::vote_against(who, nonce, src_id, call)
        }

        /// Evaluate the state of a proposal given the current vote threshold.
        ///
        /// A proposal with enough votes will be either executed or cancelled, and the status
        /// will be updated accordingly.
        ///
        /// # <weight>
        /// - weight of proposed call, regardless of whether execution is performed
        /// # </weight>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::eval_vote_state(proposal.get_dispatch_info().weight))]
        pub fn eval_vote_state(
            origin: OriginFor<T>,
            nonce: DepositNonce,
            src_id: ChainId,
            proposal: Box<<T as Config>::Proposal>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            Self::try_resolve_proposal(nonce, src_id, proposal)
        }
    }
} // end of 'pallet' module

// ----------------------------------------------------------------------------
// Pallet implementation block
// ----------------------------------------------------------------------------

// Chain bridge pallet implementation block.
//
// This main implementation block contains two categories of functions, namely:
// - Public functions: These are functions that are `pub` and generally fall into
//   inspector functions that do not write to storage and operation functions that do.
// - Private functions: These are private helpers or utilities that cannot be called
//   from other pallets.
impl<T: Config> Pallet<T> {
    // Public immutables and private mutables functions

    /// Provides an AccountId for the pallet.
    /// This is used both as an origin check and deposit/withdrawal account.
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }

    pub fn ensure_admin(o: T::Origin) -> DispatchResult {
        T::AdminOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }

    /// Checks if who is a relayer
    pub fn is_relayer(who: &T::AccountId) -> bool {
        Self::get_relayers(who)
    }

    /// Asserts if a resource is registered
    pub fn resource_exists(id: ResourceId) -> bool {
        return Self::get_resources(id) != None;
    }

    /// Checks if a chain exists as a whitelisted destination
    pub fn chain_whitelisted(id: ChainId) -> bool {
        return Self::get_chains(id) != None;
    }

    /// Increments the deposit nonce for the specified chain ID
    fn bump_nonce(id: ChainId) -> DepositNonce {
        let nonce = Self::get_chains(id).unwrap_or_default() + 1;
        <ChainNonces<T>>::insert(id, nonce);
        nonce
    }

    // *** Admin methods ***

    /// Set a new voting threshold
    pub fn set_relayer_threshold(threshold: u32) -> DispatchResult {
        ensure!(threshold > 0, Error::<T>::InvalidThreshold);
        <RelayerVoteThreshold<T>>::put(threshold);
        Self::deposit_event(Event::RelayerThresholdChanged(threshold));
        Ok(())
    }

    /// Register a method for a resource Id, enabling associated transfers
    pub fn register_resource(id: ResourceId, method: Vec<u8>) -> DispatchResult {
        <Resources<T>>::insert(id, method);
        Ok(())
    }

    /// Removes a resource ID, disabling associated transfer
    pub fn unregister_resource(id: ResourceId) -> DispatchResult {
        <Resources<T>>::remove(id);
        Ok(())
    }

    /// Whitelist a chain ID for transfer
    pub fn whitelist(id: ChainId) -> DispatchResult {
        // Cannot whitelist this chain
        ensure!(id != T::ChainId::get(), Error::<T>::InvalidChainId);
        // Cannot whitelist with an existing entry
        ensure!(
            !Self::chain_whitelisted(id),
            Error::<T>::ChainAlreadyWhitelisted
        );
        <ChainNonces<T>>::insert(&id, 0);
        Self::deposit_event(Event::ChainWhitelisted(id));
        Ok(())
    }

    /// Adds a new relayer to the set
    pub fn register_relayer(relayer: T::AccountId) -> DispatchResult {
        ensure!(
            !Self::is_relayer(&relayer),
            Error::<T>::RelayerAlreadyExists
        );
        <Relayers<T>>::insert(&relayer, true);
        <RelayerCount<T>>::mutate(|i| *i += 1);

        Self::deposit_event(Event::RelayerAdded(relayer));
        Ok(())
    }

    /// Removes a relayer from the set
    pub fn unregister_relayer(relayer: T::AccountId) -> DispatchResult {
        ensure!(Self::is_relayer(&relayer), Error::<T>::RelayerInvalid);
        <Relayers<T>>::remove(&relayer);
        <RelayerCount<T>>::mutate(|i| *i -= 1);
        Self::deposit_event(Event::RelayerRemoved(relayer));
        Ok(())
    }

    // Proposal voting and execution methods

    /// Commits a vote for a proposal. If the proposal doesn't exist it will be created.
    fn commit_vote(
        who: T::AccountId,
        nonce: DepositNonce,
        src_id: ChainId,
        prop: Box<T::Proposal>,
        in_favour: bool,
    ) -> DispatchResult {
        let now = <frame_system::Pallet<T>>::block_number();
        let mut votes = match <Votes<T>>::get(src_id, (nonce, prop.clone())) {
            Some(v) => v,
            None => {
                let mut v = ProposalVotes::default();
                v.expiry = now + T::ProposalLifetime::get();
                v
            }
        };

        // Ensure the proposal isn't complete and relayer hasn't already voted
        ensure!(!votes.is_complete(), Error::<T>::ProposalAlreadyComplete);
        ensure!(!votes.is_expired(now), Error::<T>::ProposalExpired);
        ensure!(!votes.has_voted(&who), Error::<T>::RelayerAlreadyVoted);

        if in_favour {
            votes.votes_for.push(who.clone());
            Self::deposit_event(Event::VoteFor(src_id, nonce, who.clone()));
        } else {
            votes.votes_against.push(who.clone());
            Self::deposit_event(Event::VoteAgainst(src_id, nonce, who.clone()));
        }

        <Votes<T>>::insert(src_id, (nonce, prop.clone()), votes.clone());

        Ok(())
    }

    /// Attempts to finalize or cancel the proposal if the vote count allows.
    fn try_resolve_proposal(
        nonce: DepositNonce,
        src_id: ChainId,
        prop: Box<T::Proposal>,
    ) -> DispatchResult {
        if let Some(mut votes) = <Votes<T>>::get(src_id, (nonce, prop.clone())) {
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(!votes.is_complete(), Error::<T>::ProposalAlreadyComplete);
            ensure!(!votes.is_expired(now), Error::<T>::ProposalExpired);

            let status = votes.try_to_complete(Self::get_threshold(), Self::get_relayer_count());
            <Votes<T>>::insert(src_id, (nonce, prop.clone()), votes.clone());

            match status {
                ProposalStatus::Approved => Self::finalize_execution(src_id, nonce, prop),
                ProposalStatus::Rejected => Self::cancel_execution(src_id, nonce),
                _ => Ok(()),
            }
        } else {
            Err(Error::<T>::ProposalDoesNotExist)?
        }
    }

    /// Commits a vote in favour of the proposal and executes it if the vote threshold is met.
    fn vote_for(
        who: T::AccountId,
        nonce: DepositNonce,
        src_id: ChainId,
        prop: Box<T::Proposal>,
    ) -> DispatchResult {
        Self::commit_vote(who, nonce, src_id, prop.clone(), true)?;
        Self::try_resolve_proposal(nonce, src_id, prop)
    }

    /// Commits a vote against the proposal and cancels it if more than (get_relayers.len() - threshold)
    /// votes against exist.
    fn vote_against(
        who: T::AccountId,
        nonce: DepositNonce,
        src_id: ChainId,
        prop: Box<T::Proposal>,
    ) -> DispatchResult {
        Self::commit_vote(who, nonce, src_id, prop.clone(), false)?;
        Self::try_resolve_proposal(nonce, src_id, prop)
    }

    /// Execute the proposal and signals the result as an event
    fn finalize_execution(
        src_id: ChainId,
        nonce: DepositNonce,
        call: Box<T::Proposal>,
    ) -> DispatchResult {
        Self::deposit_event(Event::ProposalApproved(src_id, nonce));

        // The dispatch origin for this call must be signed
        let origin = SystemOrigin::Signed(Self::account_id()).into();
        call.dispatch(origin).map(|_| ()).map_err(|e| e.error)?;
        Self::deposit_event(Event::ProposalSucceeded(src_id, nonce));
        Ok(())
    }

    /// Cancels a proposal.
    fn cancel_execution(src_id: ChainId, nonce: DepositNonce) -> DispatchResult {
        Self::deposit_event(Event::ProposalRejected(src_id, nonce));
        Ok(())
    }

    /// Initiates a transfer of a fungible asset out of the chain. This should be called by another pallet.
    pub fn transfer_fungible(
        dest_id: ChainId,
        resource_id: ResourceId,
        to: Vec<u8>,
        amount: U256,
    ) -> DispatchResult {
        ensure!(
            Self::chain_whitelisted(dest_id),
            Error::<T>::ChainNotWhitelisted
        );
        let nonce = Self::bump_nonce(dest_id);
        Self::deposit_event(Event::FungibleTransfer(
            dest_id,
            nonce,
            resource_id,
            amount,
            to,
        ));
        Ok(())
    }
}

/// Simple ensure origin for the bridge account
pub struct EnsureBridge<T>(sp_std::marker::PhantomData<T>);

impl<T: pallet::Config> EnsureOrigin<T::Origin> for EnsureBridge<T> {
    type Success = T::AccountId;

    fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
        let bridge_id = T::PalletId::get().into_account();
        o.into().and_then(|o| match o {
            SystemOrigin::Signed(who) if who == bridge_id => Ok(bridge_id),
            r => Err(T::Origin::from(r)),
        })
    }

    /// Returns an origin capable of passing the `try_origin` check.
    ///
    /// ** Should be used for benchmarking only!!! **
    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> T::Origin {
        let bridge_id = T::PalletId::get().into_account();

        T::Origin::from(SystemOrigin::Signed(bridge_id))
    }
}

/// Helper function to concatenate a chain ID and some bytes to produce a resource ID.
/// The common format is (31 bytes unique ID + 1 byte chain ID).
pub fn derive_resource_id(chain: u8, id: &[u8]) -> ResourceId {
    let mut r_id: ResourceId = [0; 32];
    r_id[31] = chain; // last byte is chain id
    let range = if id.len() > 31 { 31 } else { id.len() }; // Use at most 31 bytes
    for i in 0..range {
        r_id[30 - i] = id[range - 1 - i]; // Ensure left padding for eth compatibility
    }
    return r_id;
}
