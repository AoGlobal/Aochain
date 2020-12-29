// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use sc_chain_spec::ChainSpecExtension;
use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use serde_json::json;
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig,
	DemocracyConfig,GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakerStatus,
	StakingConfig, ElectionsConfig, IndicesConfig, SocietyConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, wasm_binary_unwrap, OrganizationConfig
};
use node_runtime::Block;
use node_runtime::constants::currency::*;
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<
	GenesisConfig,
	Extensions,
>;
/// aochain testnet generator
pub fn aochain_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/colombo.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5FZdjMwHfF3aXbvasamC91xDdj6PvF76rT8KywEpwHB1FuTj
		hex!["9ac4ed83b7e8a71807536bf1a9cb0865bd419073993e7980489ca43dffa11046"].into(),
		// 5DJnpPJAiq9fmaKNnDwG9323P68p8FBj9dSyprsBigZf84aS
		hex!["36fa5e3a775b5840fc039044c9eecda9d3ae6ba55a7a33c7be4a4e3aac3d0f70"].into(),
		// 5FZtbMgrTAHvJ9KnMFLegueCJxq7LSLK8KoheTu1FBjsM6Dh
		hex!["9af6f52d87127c53286c9dd15043b9f322bd935ac7316a2a7f2fb52c1eb884c2"].unchecked_into(),
		// 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
		hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].unchecked_into(),
		// 5FZtbMgrTAHvJ9KnMFLegueCJxq7LSLK8KoheTu1FBjsM6Dh
		hex!["9af6f52d87127c53286c9dd15043b9f322bd935ac7316a2a7f2fb52c1eb884c2"].unchecked_into(),
		// 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
		hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].unchecked_into(),
	),(
		// 5HT4Si96jrM6fN3ohfFRJrXn11CbDttNui2tsHaEvFjz1vsB
		hex!["ee39de2f7678211e7cb72fec510de56d1f8a2d29dd9809f5b79c931db73c4b12"].into(),
		// 5DQ1X73256aGXLjfEEAjtQdD2JQLQGV8enTPFAd9WGozXwo3
		hex!["3af55aa368fa00254a85de996fe687faeac3387ce3859dbc7cabff18d3170833"].into(),
		// 5Fr1JrHHBJznZowxRvqSjQWNpzYCnpNPPMbNRx5LjKC7uGDR
		hex!["a7417d800df985265f9e8b2b1b384a0e70aca6dba382ac3faaeabde3be8e2119"].unchecked_into(),
		// 5HTjRDDhv1tZMDBzFVDfHm3nayrtoifBSjqaTQ3GgxunxYpF
		hex!["eebd110ef7e544f10dace1ac328c5cf1db73c8f8313e8850c332dfbbaecfdb33"].unchecked_into(),
		// 5Fr1JrHHBJznZowxRvqSjQWNpzYCnpNPPMbNRx5LjKC7uGDR
		hex!["a7417d800df985265f9e8b2b1b384a0e70aca6dba382ac3faaeabde3be8e2119"].unchecked_into(),
		// 5HTjRDDhv1tZMDBzFVDfHm3nayrtoifBSjqaTQ3GgxunxYpF
		hex!["eebd110ef7e544f10dace1ac328c5cf1db73c8f8313e8850c332dfbbaecfdb33"].unchecked_into(),
	),(
		// 5DoAgLJ7mrwDXxand6sL2XAYP15imWMYo8aehk9GXz8im71h
		hex!["4c9ed4a7a7ebd38e69cb50a8793706103b9fa1d1c3d891537f8f7d02fd672012"].into(),
		// 5F48rNcU9AGFnS1pgmtyfvGGQgHZuA7jeJDLEcuDbkhcdtuY
		hex!["84459866a19c7fadcf55599d17ba5031cfbd6fc7660e63b91d24c4deef6853f8"].into(),
		// 5DqtbLdHdQiLTzZHtTcDyWjqpxSKNjWvEzW4jLuLMt5e7tMH
		hex!["4eb26a8de3cf2e064d8c6a8c3912aa2471c6cd6cfcf005e255b1a18acca00e19"].unchecked_into(),
		// 5FR2CVZgqcP84CDD54d3QGd28xXL7fLgSTMRyENWTRNQJmG2
		hex!["94335a3a9fc18aa1acc6a829c762778ad632250ec3e06795f3b7757835eb482f"].unchecked_into(),
		// 5DqtbLdHdQiLTzZHtTcDyWjqpxSKNjWvEzW4jLuLMt5e7tMH
		hex!["4eb26a8de3cf2e064d8c6a8c3912aa2471c6cd6cfcf005e255b1a18acca00e19"].unchecked_into(),
		// 5FR2CVZgqcP84CDD54d3QGd28xXL7fLgSTMRyENWTRNQJmG2
		hex!["94335a3a9fc18aa1acc6a829c762778ad632250ec3e06795f3b7757835eb482f"].unchecked_into(),
	),(
		 // 5EqyBCbDWbp8Erxec7TauCeuxvUYaKiqmqAfEKsrM88b1wWA
		 hex!["7afe148732d7e363bd77e08cddd4a6cbf93cdd985e10942aba9c43ff799e747e"].into(),
		 // 5GfZ3ysS1KudB6B8mM45SmQfm73ZBx9itGTETLuHkmdFk8Si
		 hex!["cb84e201eb5e6333ec472e3284ed0343e8c4b4f81f60733d331fcb521242e1ef"].into(),
		 // 5CsRhpzmw5zW4co8MPafW2Kt84Cc2HGMxNzBxnPE5uAaJATH
		 hex!["23a228fce8f7e79f3b4350dd6458d15f626c3ecfbf71dcba5f5766fae7da2e5c"].unchecked_into(),
		 // 5Cth6WcUmy2ZMFUthwi1YqGyDHuWs1qPFnQL82xWrD9QohKJ
		 hex!["2499376e7db1c15be07b187f8a34d858b0bd4edd91b12c3f7570de95b4c0d900"].unchecked_into(),
		 // 5CsRhpzmw5zW4co8MPafW2Kt84Cc2HGMxNzBxnPE5uAaJATH
		 hex!["23a228fce8f7e79f3b4350dd6458d15f626c3ecfbf71dcba5f5766fae7da2e5c"].unchecked_into(),
		 // 5Cth6WcUmy2ZMFUthwi1YqGyDHuWs1qPFnQL82xWrD9QohKJ
		 hex!["2499376e7db1c15be07b187f8a34d858b0bd4edd91b12c3f7570de95b4c0d900"].unchecked_into(),
	 )];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
		"6e2da89232c20b8fa01185e7045de187ddb0d496b516b04373a6018288282b67"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		root_key,
		Some(endowed_accounts),
		false,
	)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Colombo",
		"colombo",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
			.expect("Staging telemetry url is valid; qed")),
		Some("tcom"),
		Some(json!({"tokenDecimals": 10, "tokenSymbol": "TCOM", "ss58Format": 98}).as_object()
			.expect("network properties generation can not fail; qed")
			.to_owned()),
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 1_000_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 10 + 10,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		pallet_collective_Instance3: Some(OrganizationConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
		}),
		pallet_sudo: Some(SudoConfig {
			key: root_key,
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_society: Some(SocietyConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			pot: 0,
			max_members: 999,
		}),
		pallet_vesting: Some(Default::default()),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		Some(json!({"tokenDecimals": 10, "tokenSymbol": "TCOM", "ss58Format": 98}).as_object()
			.expect("network properties generation can not fail; qed")
			.to_owned()),
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}
