
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use soda_core::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};
use soda_core::ports::driven::soda_machine_repository_port::{SodaMachineRepository, RepositoryError};

type SharedMachines = Arc<Mutex<HashMap<SodaMachineId, SodaMachine>>>;

pub struct InMemorySodaMachineRepository {
    machines: SharedMachines,
}

impl InMemorySodaMachineRepository {
    pub fn new() -> Self {
        InMemorySodaMachineRepository {
            machines: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SodaMachineRepository for InMemorySodaMachineRepository {
    async fn find_by_id(&self, id: SodaMachineId) -> Result<Option<SodaMachine>, RepositoryError> {
        let machines = self.machines.lock().map_err(|e| {
            RepositoryError::ConnectionError(format!("Mutex poisoned: {}", e))
        })?;
        let result = machines.get(&id).cloned();
        // println!("find_by_id result: {:?}", result); // <- log do resultado

        Ok(result)
    }

    async fn save(&self, machine: &SodaMachine) -> Result<(), RepositoryError> {
        let mut machines = self.machines.lock().map_err(|e| {
            RepositoryError::ConnectionError(format!("Mutex poisoned: {}", e))
        })?;
        machines.insert(machine.id(), machine.clone());
        Ok(())
    }

    async fn create(&self, machine: &SodaMachine) -> Result<(), RepositoryError> {
        let mut machines = self.machines.lock().map_err(|e| {
            RepositoryError::ConnectionError(format!("Mutex poisoned: {}", e))
        })?;
        if machines.contains_key(&machine.id()) {
            return Err(RepositoryError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Machine already exists",
            ))));
        }
        machines.insert(machine.id(), machine.clone());
        Ok(())
    }
}
