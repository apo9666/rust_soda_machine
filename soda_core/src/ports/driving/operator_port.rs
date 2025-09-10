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
