use async_trait::async_trait;
use crate::domain::value_objects::soda::Soda;
use crate::domain::aggregates::soda_machine::{SodaMachineError, SodaMachineId};

#[derive(Debug)]
pub enum OperatorError {
    MachineError(SodaMachineError),
    SodaMachineNotFound(SodaMachineId),
    RepositoryUnavailable(String),
    RepositoryFailure(String),
    Validation(String),
}

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorError::MachineError(e) => write!(f, "Machine error: {}", e),
            OperatorError::SodaMachineNotFound(id) => write!(f, "Soda machine not found: {:?}", id),
            OperatorError::RepositoryUnavailable(msg) => write!(f, "Repository unavailable: {}", msg),
            OperatorError::RepositoryFailure(msg) => write!(f, "Repository failure: {}", msg),
            OperatorError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for OperatorError {}

#[async_trait]
pub trait OperatorPort {
    async fn create_new_machine(&self, machine_id: u32, max_slots: u32) -> Result<(), OperatorError>;
    async fn configure_slot(
        &self,
        machine_id: u32,
        slot_id: u32,
        capacity: u32,
        soda: Soda
    ) -> Result<(), OperatorError>;
    async fn refill_slot(&self, machine_id: u32, slot_id: u32, quantity: u32) -> Result<(), OperatorError>;
    async fn get_machine_status(&self, machine_id: u32) -> Result<String, OperatorError>;
}
