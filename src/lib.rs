pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;

// Only used for easier importing
pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};

#[cw_orch::interface(
    crate::msg::InstantiateMsg,
    crate::msg::ExecuteMsg,
    crate::msg::QueryMsg,
    crate::msg::MigrateMsg
)]
pub struct CounterContract;

// This interface file should not land inside the wasm
#[cfg(not(target_arch = "wasm32"))]
mod interface;
