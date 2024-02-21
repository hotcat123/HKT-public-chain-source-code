use crate::store::ChainStoreAccess;
use crate::types::RuntimeAdapter;
use hkt_chain_primitives::error::Error;
use hkt_primitives::hash::CryptoHash;
use hkt_primitives::types::ShardId;

/// Check that epoch of block with given prev_block_hash is the first one with current protocol version.
fn is_first_epoch_with_protocol_version(
    runtime_adapter: &dyn RuntimeAdapter,
    prev_block_hash: &CryptoHash,
) -> Result<bool, Error> {
    let prev_epoch_id = runtime_adapter.get_prev_epoch_id_from_prev_block(prev_block_hash)?;
    let epoch_id = runtime_adapter.get_epoch_id_from_prev_block(prev_block_hash)?;
    let protocol_version = runtime_adapter.get_epoch_protocol_version(&epoch_id)?;
    let prev_epoch_protocol_version = runtime_adapter.get_epoch_protocol_version(&prev_epoch_id)?;
    Ok(protocol_version != prev_epoch_protocol_version)
}

/// Check that block is the first one with existing chunk for the given shard in the chain with its protocol version.
/// We assume that current block contain the chunk for shard with the given id.
pub fn check_if_block_is_first_with_chunk_of_version(
    chain_store: &dyn ChainStoreAccess,
    runtime_adapter: &dyn RuntimeAdapter,
    prev_block_hash: &CryptoHash,
    shard_id: ShardId,
) -> Result<bool, Error> {
    // Check that block belongs to the first epoch with current protocol version
    // to avoid get_epoch_id_of_last_block_with_chunk call in the opposite case
    if is_first_epoch_with_protocol_version(runtime_adapter, prev_block_hash)? {
        // Compare only epochs because we already know that current epoch is the first one with current protocol version
        // convert shard id to shard id of previous epoch because number of shards may change
        let shard_id = runtime_adapter.get_prev_shard_ids(prev_block_hash, vec![shard_id])?[0];
        let prev_epoch_id = chain_store.get_epoch_id_of_last_block_with_chunk(
            runtime_adapter,
            prev_block_hash,
            shard_id,
        )?;
        let epoch_id = runtime_adapter.get_epoch_id_from_prev_block(prev_block_hash)?;
        Ok(prev_epoch_id != epoch_id)
    } else {
        Ok(false)
    }
}