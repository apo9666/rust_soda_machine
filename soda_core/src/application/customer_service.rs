use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::aggregates::soda_machine::{SodaMachineId};
use crate::domain::entities::slot::SlotId;
use crate::domain::value_objects::money::Money;
use crate::ports::driving::customer_port::{CustomerPort, AvailableSodaDTO, CustomerError};
use crate::ports::driven::soda_machine_repository_port::{SodaMachineRepository, RepositoryError};

impl From<RepositoryError> for CustomerError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::ConnectionError(msg) => CustomerError::RepositoryUnavailable(msg),
            RepositoryError::Other(e) => CustomerError::RepositoryFailure(e.to_string()),
        }
    }
}

pub struct CustomerService {
    repository: Arc<dyn SodaMachineRepository>,
}

impl CustomerService {
    pub fn new(repository: Arc<dyn SodaMachineRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CustomerPort for CustomerService {
    async fn list_available_sodas(&self, machine_id: u32) -> Result<Vec<AvailableSodaDTO>, CustomerError> {
        let machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(CustomerError::from)?
            .ok_or(CustomerError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;

        let available_sodas = machine.get_available_sodas().into_iter().map(|(slot_id, soda)| {
            AvailableSodaDTO {
                slot_id: slot_id.value(),
                soda_name: soda.name().to_string(),
                price: format!("{:.2}", soda.price().as_decimal()),
            }
        }).collect();
       
        Ok(available_sodas)
    }

    async fn insert_money(&self, machine_id: u32, amount: Money) -> Result<(), CustomerError> {
        let mut machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(CustomerError::from)?
            .ok_or(CustomerError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;

        machine.insert_money(amount).map_err(CustomerError::MachineError)?;
       
        self.repository.save(&machine).await.map_err(CustomerError::from)?;

        Ok(())
    }

    async fn buy_soda(&self, machine_id: u32, slot_id: u32) -> Result<(), CustomerError> {
        let mut machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(CustomerError::from)?
            .ok_or(CustomerError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;
       
        machine.dispense_soda(SlotId::new(slot_id)).map_err(CustomerError::MachineError)?;

        self.repository.save(&machine).await.map_err(CustomerError::from)?;
       
        Ok(())
    }
   
    async fn request_money_back(&self, machine_id: u32) -> Result<Money, CustomerError> {
        let mut machine = self.repository
            .find_by_id(SodaMachineId::new(machine_id))
            .await
            .map_err(CustomerError::from)?
            .ok_or(CustomerError::SodaMachineNotFound(SodaMachineId::new(machine_id)))?;
       
        let inserted_money = machine.inserted_money();
        machine.return_money().map_err(CustomerError::MachineError)?;
       
        self.repository.save(&machine).await.map_err(CustomerError::from)?;

        Ok(inserted_money)
    }
}
