use async_trait::async_trait;
use std::error::Error;
use std::fmt;

use crate::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};

#[derive(Debug)]
pub enum RepositoryError {
    ConnectionError(String),
    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            RepositoryError::Other(err) => write!(f, "Repository error: {}", err),
        }
    }
}

impl Error for RepositoryError {}

#[async_trait]
pub trait SodaMachineRepository: Send + Sync {
    async fn find_by_id(&self, id: SodaMachineId) -> Result<Option<SodaMachine>, RepositoryError>;
    async fn save(&self, machine: &SodaMachine) -> Result<(), RepositoryError>;
    async fn create(&self, machine: &SodaMachine) -> Result<(), RepositoryError>;
}