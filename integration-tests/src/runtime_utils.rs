use std::collections::HashSet;

use hkt_chain_configs::Genesis;
use hkt_primitives::shard_layout::ShardUId;
use hkt_primitives::state_record::{state_record_to_account_id, StateRecord};
use hkt_primitives::types::AccountId;
use hkt_primitives::types::StateRoot;
use hkt_store::test_utils::create_tries_complex;
use hkt_store::{ShardTries, TrieUpdate};
use hktcore::config::GenesisExt;
use node_runtime::config::RuntimeConfig;
use node_runtime::{state_viewer::TrieViewer, Runtime};
use testlib::runtime_utils::{add_test_contract, alice_account, bob_account};

pub const TEST_SHARD_UID: ShardUId = ShardUId { version: 1, shard_id: 0 };

pub fn get_runtime_and_trie() -> (Runtime, ShardTries, StateRoot) {
    let mut genesis = Genesis::test_sharded_new_version(
        vec![alice_account(), bob_account(), "carol.hkt".parse().unwrap()],
        3,
        vec![3],
    );
    add_test_contract(&mut genesis, &"test.contract".parse().unwrap());
    get_runtime_and_trie_from_genesis(&genesis)
}

pub fn get_test_trie_viewer() -> (TrieViewer, TrieUpdate) {
    let (_, tries, root) = get_runtime_and_trie();
    let trie_viewer = TrieViewer::default();
    let state_update = tries.new_trie_update(TEST_SHARD_UID, root);
    (trie_viewer, state_update)
}

pub fn get_runtime_and_trie_from_genesis(genesis: &Genesis) -> (Runtime, ShardTries, StateRoot) {
    let shard_layout = &genesis.config.shard_layout;
    let tries = create_tries_complex(shard_layout.version(), shard_layout.num_shards());
    let runtime = Runtime::new();
    let mut account_ids: HashSet<AccountId> = HashSet::new();
    genesis.for_each_record(|record: &StateRecord| {
        account_ids.insert(state_record_to_account_id(record).clone());
    });
    let genesis_root = runtime.apply_genesis_state(
        tries.clone(),
        0,
        &genesis
            .config
            .validators
            .iter()
            .map(|account_info| {
                (
                    account_info.account_id.clone(),
                    account_info.public_key.clone(),
                    account_info.amount,
                )
            })
            .collect::<Vec<_>>(),
        genesis,
        &RuntimeConfig::test(),
        account_ids,
    );
    (runtime, tries, genesis_root)
}