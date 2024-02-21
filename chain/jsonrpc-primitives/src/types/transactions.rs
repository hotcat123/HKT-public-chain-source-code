use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RpcBroadcastTransactionRequest {
    pub signed_transaction: hkt_primitives::transaction::SignedTransaction,
}

#[derive(Debug)]
pub struct RpcTransactionStatusCommonRequest {
    pub transaction_info: TransactionInfo,
}

#[derive(Clone, Debug)]
pub enum TransactionInfo {
    Transaction(hkt_primitives::transaction::SignedTransaction),
    TransactionId {
        hash: hkt_primitives::hash::CryptoHash,
        account_id: hkt_primitives::types::AccountId,
    },
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcTransactionError {
    #[error("An error happened during transaction execution: {context:?}")]
    InvalidTransaction {
        #[serde(skip_serializing)]
        context: hkt_primitives::errors::InvalidTxError,
    },
    #[error("Node doesn't track this shard. Cannot determine whether the transaction is valid")]
    DoesNotTrackShard,
    #[error("Transaction with hash {transaction_hash} was routed")]
    RequestRouted { transaction_hash: hkt_primitives::hash::CryptoHash },
    #[error("Transaction {requested_transaction_hash} doesn't exist")]
    UnknownTransaction { requested_transaction_hash: hkt_primitives::hash::CryptoHash },
    #[error("The node reached its limits. Try again later. More details: {debug_info}")]
    InternalError { debug_info: String },
    #[error("Timeout")]
    TimeoutError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcTransactionResponse {
    #[serde(flatten)]
    pub final_execution_outcome: hkt_primitives::views::FinalExecutionOutcomeViewEnum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcBroadcastTxSyncResponse {
    pub transaction_hash: hkt_primitives::hash::CryptoHash,
}

impl From<RpcTransactionError> for crate::errors::RpcError {
    fn from(error: RpcTransactionError) -> Self {
        let error_data = match &error {
            RpcTransactionError::InvalidTransaction { context } => {
                if let Ok(value) =
                    serde_json::to_value(crate::errors::ServerError::TxExecutionError(
                        hkt_primitives::errors::TxExecutionError::InvalidTxError(context.clone()),
                    ))
                {
                    value
                } else {
                    Value::String(error.to_string())
                }
            }
            _ => Value::String(error.to_string()),
        };

        let error_data_value = match serde_json::to_value(error) {
            Ok(value) => value,
            Err(err) => {
                return Self::new_internal_error(
                    None,
                    format!("Failed to serialize RpcTransactionError: {:?}", err),
                )
            }
        };

        Self::new_internal_or_handler_error(Some(error_data), error_data_value)
    }
}
