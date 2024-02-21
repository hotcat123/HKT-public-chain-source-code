use std::collections::HashMap;

use genesis_populate::get_account_id;
use hkt_crypto::{InMemorySigner, KeyType};
use hkt_primitives::hash::CryptoHash;
use hkt_primitives::transaction::{Action, FunctionCallAction, SignedTransaction};
use hkt_primitives::types::AccountId;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

/// A helper to create transaction for processing by a `TestBed`.
#[derive(Clone)]
pub(crate) struct TransactionBuilder {
    accounts: Vec<AccountId>,
    nonces: HashMap<AccountId, u64>,
    unused_accounts: Vec<usize>,
    unused_index: usize,
}

impl TransactionBuilder {
    pub(crate) fn new(accounts: Vec<AccountId>) -> TransactionBuilder {
        let n = accounts.len();
        let mut rng = rand::thread_rng();
        let mut unused_accounts: Vec<usize> = Vec::from_iter(0..n);
        unused_accounts.shuffle(&mut rng);
        let unused_index: usize = 0;

        TransactionBuilder { accounts, nonces: HashMap::new(), unused_accounts, unused_index }
    }

    pub(crate) fn transaction_from_actions(
        &mut self,
        sender: AccountId,
        receiver: AccountId,
        actions: Vec<Action>,
    ) -> SignedTransaction {
        let signer = InMemorySigner::from_seed(sender.clone(), KeyType::ED25519, sender.as_ref());
        let nonce = self.nonce(&sender);

        SignedTransaction::from_actions(
            nonce as u64,
            sender.clone(),
            receiver,
            &signer,
            actions,
            CryptoHash::default(),
        )
    }

    pub(crate) fn transaction_from_function_call(
        &mut self,
        sender: AccountId,
        method: &str,
        args: Vec<u8>,
    ) -> SignedTransaction {
        let receiver = sender.clone();
        let actions = vec![Action::FunctionCall(FunctionCallAction {
            method_name: method.to_string(),
            args,
            gas: 10u64.pow(18),
            deposit: 0,
        })];
        self.transaction_from_actions(sender, receiver, actions)
    }

    /// Transaction that inserts a value for a given key under an account.
    /// The account must have the test contract deployed.
    pub(crate) fn account_insert_key(
        &mut self,
        account: AccountId,
        key: &[u8],
        value: &[u8],
    ) -> SignedTransaction {
        let arg = (key.len() as u64)
            .to_le_bytes()
            .into_iter()
            .chain(key.iter().cloned())
            .chain((value.len() as u64).to_le_bytes().into_iter())
            .chain(value.iter().cloned())
            .collect();

        self.transaction_from_function_call(account, "account_storage_insert_key", arg)
    }

    /// Transaction that checks existence of a given key under an account.
    /// The account must have the test contract deployed.
    pub(crate) fn account_has_key(&mut self, account: AccountId, key: &str) -> SignedTransaction {
        let arg = (key.len() as u64).to_le_bytes().into_iter().chain(key.bytes()).collect();

        self.transaction_from_function_call(account, "account_storage_has_key", arg)
    }

    pub(crate) fn rng(&mut self) -> ThreadRng {
        rand::thread_rng()
    }

    pub(crate) fn account(&mut self, account_index: u64) -> AccountId {
        get_account_id(account_index)
    }
    pub(crate) fn random_account(&mut self) -> AccountId {
        let account_index = self.rng().gen_range(0, self.accounts.len());
        self.accounts[account_index].clone()
    }
    pub(crate) fn random_unused_account(&mut self) -> AccountId {
        if self.unused_index >= self.unused_accounts.len() {
            panic!("All accounts used. Try running with a higher value for the parameter `--accounts-num <NUM>`.")
        }
        let tmp = self.unused_index;
        self.unused_index += 1;
        return self.accounts[self.unused_accounts[tmp]].clone();
    }
    pub(crate) fn random_account_pair(&mut self) -> (AccountId, AccountId) {
        let first = self.random_account();
        loop {
            let second = self.random_account();
            if first != second {
                return (first, second);
            }
        }
    }

    pub(crate) fn random_vec(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.rng().gen()).collect()
    }

    fn nonce(&mut self, account_id: &AccountId) -> u64 {
        let nonce = self.nonces.entry(account_id.clone()).or_default();
        *nonce += 1;
        *nonce
    }
}