use hkt_chain::{ChainGenesis, RuntimeAdapter};
use hkt_chain_configs::Genesis;
use hkt_client::test_utils::TestEnv;
use hkt_crypto::{InMemorySigner, KeyType, Signer};
use hkt_network::types::NetworkClientResponses;
use hkt_primitives::account::{AccessKey, AccessKeyPermission, FunctionCallPermission};
use hkt_primitives::errors::{ActionsValidationError, InvalidTxError};
use hkt_primitives::hash::CryptoHash;
use hkt_primitives::runtime::config_store::RuntimeConfigStore;
use hkt_primitives::transaction::{Action, AddKeyAction, Transaction};
use hkt_store::test_utils::create_test_store;
use hktcore::config::GenesisExt;
use hktcore::TrackedConfig;
use std::path::Path;
use std::sync::Arc;

#[test]
fn test_account_id_in_function_call_permission_upgrade() {
    let old_protocol_version =
        hkt_primitives::version::ProtocolFeature::AccountIdInFunctionCallPermission
            .protocol_version()
            - 1;
    let new_protocol_version = old_protocol_version + 1;

    // Prepare TestEnv with a contract at the old protocol version.
    let mut env = {
        let epoch_length = 5;
        let mut genesis =
            Genesis::test(vec!["test0".parse().unwrap(), "test1".parse().unwrap()], 1);
        genesis.config.epoch_length = epoch_length;
        genesis.config.protocol_version = old_protocol_version;
        let chain_genesis = ChainGenesis::new(&genesis);
        TestEnv::builder(chain_genesis)
            .runtime_adapters(vec![Arc::new(
                hktcore::NightshadeRuntime::test_with_runtime_config_store(
                    Path::new("../../../.."),
                    create_test_store(),
                    &genesis,
                    TrackedConfig::new_empty(),
                    RuntimeConfigStore::new(None),
                ),
            ) as Arc<dyn RuntimeAdapter>])
            .build()
    };

    let signer = InMemorySigner::from_seed("test0".parse().unwrap(), KeyType::ED25519, "test0");
    let tx = Transaction {
        signer_id: "test0".parse().unwrap(),
        receiver_id: "test0".parse().unwrap(),
        public_key: signer.public_key(),
        actions: vec![Action::AddKey(AddKeyAction {
            public_key: signer.public_key(),
            access_key: AccessKey {
                nonce: 1,
                permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
                    allowance: None,
                    receiver_id: "#".to_string(),
                    method_names: vec![],
                }),
            },
        })],
        nonce: 0,
        block_hash: CryptoHash::default(),
    };

    // Run the transaction, it should pass as we don't do validation at this protocol version.
    {
        let tip = env.clients[0].chain.head().unwrap();
        let signed_transaction =
            Transaction { nonce: 10, block_hash: tip.last_block_hash, ..tx.clone() }.sign(&signer);
        let res = env.clients[0].process_tx(signed_transaction, false, false);
        assert_eq!(res, NetworkClientResponses::ValidTx);
        for i in 0..3 {
            env.produce_block(0, tip.height + i + 1);
        }
    };

    env.upgrade_protocol(new_protocol_version);

    // Re-run the transaction, now it fails due to invalid account id.
    {
        let tip = env.clients[0].chain.head().unwrap();
        let signed_transaction =
            Transaction { nonce: 11, block_hash: tip.last_block_hash, ..tx }.sign(&signer);
        let res = env.clients[0].process_tx(signed_transaction, false, false);
        assert_eq!(
            res,
            NetworkClientResponses::InvalidTx(InvalidTxError::ActionsValidation(
                ActionsValidationError::InvalidAccountId { account_id: "#".to_string() }
            ))
        )
    };
}

#[test]
fn test_very_long_account_id() {
    let mut env = {
        let genesis = Genesis::test(vec!["test0".parse().unwrap(), "test1".parse().unwrap()], 1);
        let chain_genesis = ChainGenesis::new(&genesis);
        TestEnv::builder(chain_genesis)
            .runtime_adapters(vec![Arc::new(
                hktcore::NightshadeRuntime::test_with_runtime_config_store(
                    Path::new("../../../.."),
                    create_test_store(),
                    &genesis,
                    TrackedConfig::new_empty(),
                    RuntimeConfigStore::new(None),
                ),
            ) as Arc<dyn RuntimeAdapter>])
            .build()
    };

    let tip = env.clients[0].chain.head().unwrap();
    let signer = InMemorySigner::from_seed("test0".parse().unwrap(), KeyType::ED25519, "test0");
    let tx = Transaction {
        signer_id: "test0".parse().unwrap(),
        receiver_id: "test0".parse().unwrap(),
        public_key: signer.public_key(),
        actions: vec![Action::AddKey(AddKeyAction {
            public_key: signer.public_key(),
            access_key: AccessKey {
                nonce: 1,
                permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
                    allowance: None,
                    receiver_id: "A".repeat(1024),
                    method_names: vec![],
                }),
            },
        })],
        nonce: 0,
        block_hash: tip.last_block_hash,
    }
    .sign(&signer);

    let res = env.clients[0].process_tx(tx, false, false);
    assert_eq!(
        res,
        NetworkClientResponses::InvalidTx(InvalidTxError::ActionsValidation(
            ActionsValidationError::InvalidAccountId { account_id: "A".repeat(128) }
        ))
    )
}
