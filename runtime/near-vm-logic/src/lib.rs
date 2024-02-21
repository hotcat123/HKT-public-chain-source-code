#![doc = include_str!("../README.md")]

mod alt_bn128;
mod array_utils;
mod context;
mod dependencies;
pub mod gas_counter;
mod logic;
pub mod mocks;
pub(crate) mod receipt_manager;
#[cfg(test)]
mod tests;
pub mod types;
mod utils;

pub use context::VMContext;
pub use dependencies::{External, MemoryLike, ValuePtr};
pub use logic::{VMLogic, VMOutcome};
pub use hkt_primitives_core::config::*;
pub use hkt_primitives_core::profile;
pub use hkt_primitives_core::types::ProtocolVersion;
pub use hkt_vm_errors::{HostError, VMLogicError};
pub use receipt_manager::ReceiptMetadata;
pub use types::ReturnData;

pub use gas_counter::with_ext_cost_counter;