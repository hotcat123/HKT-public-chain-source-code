use byteorder::{ByteOrder, LittleEndian};
use once_cell::sync::Lazy;

use hkt_chain_configs::Genesis;
use hkt_primitives::account::Account;
use hkt_primitives::hash::{hash, CryptoHash};
use hkt_primitives::state_record::StateRecord;
use hkt_primitives::types::AccountId;

pub fn alice_account() -> AccountId {
    "alice.hkt".parse().unwrap()
}
pub fn bob_account() -> AccountId {
    "bob.hkt".parse().unwrap()
}
pub fn eve_dot_alice_account() -> AccountId {
    "eve.alice.hkt".parse().unwrap()
}

pub fn x_dot_y_dot_alice_account() -> AccountId {
    "x.y.alice.hkt".parse().unwrap()
}

static DEFAULT_TEST_CONTRACT_HASH: Lazy<CryptoHash> =
    Lazy::new(|| hash(hkt_test_contracts::rs_contract()));

pub fn add_test_contract(genesis: &mut Genesis, account_id: &AccountId) {
    let mut is_account_record_found = false;
    let records = genesis.force_read_records().as_mut();
    for record in records.iter_mut() {
        if let StateRecord::Account { account_id: record_account_id, ref mut account } = record {
            if record_account_id == account_id {
                is_account_record_found = true;
                account.set_code_hash(*DEFAULT_TEST_CONTRACT_HASH);
            }
        }
    }
    if !is_account_record_found {
        records.push(StateRecord::Account {
            account_id: account_id.clone(),
            account: Account::new(0, 0, *DEFAULT_TEST_CONTRACT_HASH, 0),
        });
    }
    records.push(StateRecord::Contract {
        account_id: account_id.clone(),
        code: hkt_test_contracts::rs_contract().to_vec(),
    });
}

pub fn encode_int(val: i32) -> [u8; 4] {
    let mut tmp = [0u8; 4];
    LittleEndian::write_i32(&mut tmp, val);
    tmp
}
