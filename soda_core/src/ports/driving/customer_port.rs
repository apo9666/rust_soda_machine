use async_trait::async_trait;
use crate::domain::aggregates::soda_machine::{SodaMachineError, SodaMachineId};
use crate::domain::value_objects::money::Money;

#[derive(Debug, Clone, PartialEq)]
pub struct AvailableSodaDTO {
    pub slot_id: u32,
    pub soda_name: String,
    pub price: String,
}

#[derive(Debug)]
pub enum CustomerError {
    MachineError(SodaMachineError),
    SodaMachineNotFound(SodaMachineId),
    RepositoryUnavailable(String),
    RepositoryFailure(String),
    Validation(String),
}

#[async_trait]
pub trait CustomerPort {
    async fn list_available_sodas(&self, machine_id: u32) -> Result<Vec<AvailableSodaDTO>, CustomerError>;
    async fn insert_money(&self, machine_id: u32, amount: Money) -> Result<(), CustomerError>;
    async fn buy_soda(&self, machine_id: u32, slot_id: u32) -> Result<(), CustomerError>;
    async fn request_money_back(&self, machine_id: u32) -> Result<Money, CustomerError>;
}
