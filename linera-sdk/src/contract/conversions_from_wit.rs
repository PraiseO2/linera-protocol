// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen`] to types declared in [`linera-sdk`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, BlockHeight, TimeDelta, Timestamp},
    http,
    identifiers::{ApplicationId, BytecodeId, ChainId, MessageId, Owner},
    ownership::{
        ChainOwnership, ChangeApplicationPermissionsError, CloseChainError, TimeoutConfig,
    },
};

use super::wit::contract_system_api as wit_system_api;

impl From<wit_system_api::Timestamp> for Timestamp {
    fn from(timestamp: wit_system_api::Timestamp) -> Self {
        Timestamp::from(timestamp.inner0)
    }
}

impl From<wit_system_api::MessageId> for MessageId {
    fn from(message_id: wit_system_api::MessageId) -> Self {
        MessageId {
            chain_id: message_id.chain_id.into(),
            height: BlockHeight(message_id.height.inner0),
            index: message_id.index,
        }
    }
}

impl From<wit_system_api::ApplicationId> for ApplicationId {
    fn from(application_id: wit_system_api::ApplicationId) -> Self {
        ApplicationId {
            bytecode_id: application_id.bytecode_id.into(),
            creation: application_id.creation.into(),
        }
    }
}

impl From<wit_system_api::BytecodeId> for BytecodeId {
    fn from(bytecode_id: wit_system_api::BytecodeId) -> Self {
        BytecodeId::new(
            bytecode_id.contract_blob_hash.into(),
            bytecode_id.service_blob_hash.into(),
        )
    }
}

impl From<wit_system_api::ChainId> for ChainId {
    fn from(chain_id: wit_system_api::ChainId) -> Self {
        ChainId(chain_id.inner0.into())
    }
}

impl From<wit_system_api::BlockHeight> for BlockHeight {
    fn from(block_height: wit_system_api::BlockHeight) -> Self {
        BlockHeight(block_height.inner0)
    }
}

impl From<wit_system_api::Amount> for Amount {
    fn from(balance: wit_system_api::Amount) -> Self {
        let (lower_half, upper_half) = balance.inner0;
        let value = ((upper_half as u128) << 64) | (lower_half as u128);
        Amount::from_attos(value)
    }
}

impl From<wit_system_api::CryptoHash> for CryptoHash {
    fn from(crypto_hash: wit_system_api::CryptoHash) -> Self {
        CryptoHash::from([
            crypto_hash.part1,
            crypto_hash.part2,
            crypto_hash.part3,
            crypto_hash.part4,
        ])
    }
}

impl From<wit_system_api::Owner> for Owner {
    fn from(owner: wit_system_api::Owner) -> Self {
        Owner(owner.inner0.into())
    }
}

impl From<wit_system_api::TimeoutConfig> for TimeoutConfig {
    fn from(guest: wit_system_api::TimeoutConfig) -> TimeoutConfig {
        let wit_system_api::TimeoutConfig {
            fast_round_duration,
            base_timeout,
            timeout_increment,
            fallback_duration,
        } = guest;
        TimeoutConfig {
            fast_round_duration: fast_round_duration.map(TimeDelta::from),
            base_timeout: base_timeout.into(),
            timeout_increment: timeout_increment.into(),
            fallback_duration: fallback_duration.into(),
        }
    }
}

impl From<wit_system_api::TimeDelta> for TimeDelta {
    fn from(guest: wit_system_api::TimeDelta) -> Self {
        TimeDelta::from_micros(guest.inner0)
    }
}

impl From<wit_system_api::ChainOwnership> for ChainOwnership {
    fn from(guest: wit_system_api::ChainOwnership) -> ChainOwnership {
        let wit_system_api::ChainOwnership {
            super_owners,
            owners,
            multi_leader_rounds,
            open_multi_leader_rounds,
            timeout_config,
        } = guest;
        ChainOwnership {
            super_owners: super_owners.into_iter().map(Into::into).collect(),
            owners: owners
                .into_iter()
                .map(|(owner, weight)| (owner.into(), weight))
                .collect(),
            multi_leader_rounds,
            open_multi_leader_rounds,
            timeout_config: timeout_config.into(),
        }
    }
}

impl From<wit_system_api::CloseChainError> for CloseChainError {
    fn from(guest: wit_system_api::CloseChainError) -> Self {
        match guest {
            wit_system_api::CloseChainError::NotPermitted => CloseChainError::NotPermitted,
        }
    }
}

impl From<wit_system_api::ChangeApplicationPermissionsError> for ChangeApplicationPermissionsError {
    fn from(guest: wit_system_api::ChangeApplicationPermissionsError) -> Self {
        match guest {
            wit_system_api::ChangeApplicationPermissionsError::NotPermitted => {
                ChangeApplicationPermissionsError::NotPermitted
            }
        }
    }
}

impl From<wit_system_api::HttpResponse> for http::Response {
    fn from(guest: wit_system_api::HttpResponse) -> http::Response {
        http::Response {
            status: guest.status,
            headers: guest.headers.into_iter().map(http::Header::from).collect(),
            body: guest.body,
        }
    }
}

impl From<wit_system_api::HttpHeader> for http::Header {
    fn from(guest: wit_system_api::HttpHeader) -> http::Header {
        http::Header::new(guest.name, guest.value)
    }
}
