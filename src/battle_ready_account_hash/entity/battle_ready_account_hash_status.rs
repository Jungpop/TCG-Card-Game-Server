use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BattleReadyAccountHashStatus {
    WAIT,
    PREPARE,
    PREPARE_PROCESS,
    SUCCESS,
    FAIL,
}