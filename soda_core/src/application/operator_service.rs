use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};
use crate::domain::entities::slot::SlotId;
use crate::domain::value_objects::soda::Soda;
use crate::ports::driving::operator_port::{OperatorPort, OperatorError};
use crate::ports::driven::soda_machine_repository_port::{SodaMachineRepository, RepositoryError};

impl From<RepositoryError> for OperatorError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::ConnectionError(msg) => OperatorError::RepositoryUnavailable(msg),
            RepositoryError::Other(e) => OperatorError::RepositoryFailure(e.to_string()),
        }
    }
}

pub struct OperatorService {
    repository: Arc<dyn SodaMachineRepository>,
}

impl OperatorService {
    pub fn new(repository: Arc<dyn SodaMachineRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl OperatorPort for OperatorService {
    async fn create_new_machine(&self, machine_id: u32, max_slots: u32) -> Result<(), OperatorError> {
        let machine = SodaMachine::new(SodaMachineId::new(machine_id), max_slots)
            .map_err(OperatorError::MachineError)?;
       
        self.repository.create(&machine).await.map_err(OperatorError::from)?;
       
        Ok(())
    }
   
    async fn configure_slot(
        &self,
        machine_id: u32,
        slot_id: u32,
        capacity: u32,
        soda: Soda
    ) -> Result<(), OperatorError> {
        let mut machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(OperatorError::from)?
            .ok_or(OperatorError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;

        if machine.get_slot(SlotId::new(slot_id)).is_none() {
            machine.add_slot(SlotId::new(slot_id), capacity).map_err(OperatorError::MachineError)?;
        }
       
        machine.configure_slot(SlotId::new(slot_id), soda).map_err(OperatorError::MachineError)?;

        self.repository.save(&machine).await.map_err(OperatorError::from)?;

        Ok(())
    }

    async fn refill_slot(&self, machine_id: u32, slot_id: u32, quantity: u32) -> Result<(), OperatorError> {
        let mut machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(OperatorError::from)?
            .ok_or(OperatorError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;

        machine.refill_slot(SlotId::new(slot_id), quantity).map_err(OperatorError::MachineError)?;
       
        self.repository.save(&machine).await.map_err(OperatorError::from)?;
       
        Ok(())
    }

    async fn get_machine_status(&self, machine_id: u32) -> Result<String, OperatorError> {
        let machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(OperatorError::from)?
            .ok_or(OperatorError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;

        Ok(machine.status_summary())
    }
}

