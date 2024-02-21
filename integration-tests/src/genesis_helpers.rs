use std::sync::Arc;

use tempfile::tempdir;

use hkt_chain::{Chain, ChainGenesis, DoomslugThresholdMode};
use hkt_chain_configs::Genesis;
use hkt_primitives::block::{Block, BlockHeader};
use hkt_primitives::hash::CryptoHash;
use hkt_store::test_utils::create_test_store;
use hktcore::NightshadeRuntime;

/// Compute genesis hash from genesis.
pub fn genesis_hash(genesis: &Genesis) -> CryptoHash {
    *genesis_header(genesis).hash()
}

/// Utility to generate genesis header from config for testing purposes.
pub fn genesis_header(genesis: &Genesis) -> BlockHeader {
    let dir = tempdir().unwrap();
    let store = create_test_store();
    let chain_genesis = ChainGenesis::new(genesis);
    let runtime = Arc::new(NightshadeRuntime::test(dir.path(), store, genesis));
    let chain =
        Chain::new(runtime, &chain_genesis, DoomslugThresholdMode::TwoThirds, true).unwrap();
    chain.genesis().clone()
}

/// Utility to generate genesis header from config for testing purposes.
pub fn genesis_block(genesis: &Genesis) -> Block {
    let dir = tempdir().unwrap();
    let store = create_test_store();
    let chain_genesis = ChainGenesis::new(genesis);
    let runtime = Arc::new(NightshadeRuntime::test(dir.path(), store, genesis));
    let chain =
        Chain::new(runtime, &chain_genesis, DoomslugThresholdMode::TwoThirds, true).unwrap();
    chain.get_block(&chain.genesis().hash().clone()).unwrap()
}
