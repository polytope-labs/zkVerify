// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_groth16_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 31.0.0
//! DATE: 2024-06-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/mdamico/devel/NH-core/target/production/nh-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-groth16-verifier
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/mdamico/devel/NH-core/HEADER-APACHE2
// --output
// verifiers/groth16/src/weight.rs
// --template
// /home/mdamico/devel/NH-core/node/hl-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub struct NHWeight<T>(PhantomData<T>);

// For backwards compatibility and tests.
impl<T: frame_system::Config>  pallet_groth16_verifier::WeightInfo for NHWeight<T> {
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 20_209_932_000 picoseconds.
        Weight::from_parts(20_000_000_000, 3537)
            // Standard Error: 5_291_865
            .saturating_add(Weight::from_parts(1_650_742_478, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 23_130_295_000 picoseconds.
        Weight::from_parts(23_000_000_000, 3537)
            // Standard Error: 2_382_201
            .saturating_add(Weight::from_parts(1_736_757_769, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(1946), added: 4421, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `663 + n * (66 ±0)`
        //  Estimated: `5411`
        // Minimum execution time: 18_268_345_000 picoseconds.
        Weight::from_parts(19_000_000_000, 5411)
            // Standard Error: 4_761_059
            .saturating_add(Weight::from_parts(1_000_946_303, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(1946), added: 4421, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `919 + n * (98 ±0)`
        //  Estimated: `5411`
        // Minimum execution time: 21_471_343_000 picoseconds.
        Weight::from_parts(22_000_000_000, 5411)
            // Standard Error: 4_010_842
            .saturating_add(Weight::from_parts(1_416_558_841, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(1946), added: 4421, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_925_592_000 picoseconds.
        Weight::from_parts(5_000_000_000, 0)
            // Standard Error: 1_175_336
            .saturating_add(Weight::from_parts(437_722_739, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(1946), added: 4421, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_243_647_000 picoseconds.
        Weight::from_parts(2_000_000_000, 0)
            // Standard Error: 368_742
            .saturating_add(Weight::from_parts(336_470_684, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}