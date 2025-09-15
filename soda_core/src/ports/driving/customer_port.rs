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

impl std::fmt::Display for CustomerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerError::MachineError(e) => write!(f, "Machine error: {}", e),
            CustomerError::SodaMachineNotFound(id) => write!(f, "Soda machine not found: {:?}", id),
            CustomerError::RepositoryUnavailable(msg) => write!(f, "Repository unavailable: {}", msg),
            CustomerError::RepositoryFailure(msg) => write!(f, "Repository failure: {}", msg),
            CustomerError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for CustomerError {}

#[async_trait]
pub trait CustomerPort {
    async fn list_available_sodas(&self, machine_id: u32) -> Result<Vec<AvailableSodaDTO>, CustomerError>;
    async fn insert_money(&self, machine_id: u32, amount: Money) -> Result<(), CustomerError>;
    async fn buy_soda(&self, machine_id: u32, slot_id: u32) -> Result<(), CustomerError>;
    async fn request_money_back(&self, machine_id: u32) -> Result<Money, CustomerError>;
}
