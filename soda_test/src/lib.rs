#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use memory_repository::InMemorySodaMachineRepository;
    use soda_core::{
        application::{
            customer_service::CustomerService,
            operator_service::OperatorService,
        },
        domain::{
            value_objects::{
                money::Money,
                soda::{Soda, SodaFlavor, SodaSize},
            },
            aggregates::soda_machine::{SodaMachine, SodaMachineId},
            entities::slot::SlotId,
        },
        ports::{
            driving::{
                customer_port::CustomerPort,
                operator_port::OperatorPort,
            },
            driven::soda_machine_repository_port::SodaMachineRepository,
        },
    };

    #[tokio::test]
    async fn test_buying_soda_returns_correct_change() {
        // Arrange
        let repository = Arc::new(InMemorySodaMachineRepository::new());
        let customer_service = Arc::new(CustomerService::new(repository.clone()));
        let machine_id = 1;
        let slot_id = 1;
        let soda_price = Money::from_cents(150);
        
        // Create and configure a soda machine with a slot
        let mut machine = SodaMachine::new(SodaMachineId::new(machine_id), 10).unwrap();
        
        // Add a slot and configure it with Cola
        machine.add_slot(SlotId::new(slot_id), 10).unwrap();
        let cola = Soda::new(
            "Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            soda_price,
            false,
            true,
        ).unwrap();
        machine.configure_slot(SlotId::new(slot_id), cola).unwrap();
        machine.refill_slot(SlotId::new(slot_id), 5).unwrap();
        
        // Store the configured machine
        repository.create(&machine).await.unwrap();
        
        // Insert enough money and a bit extra to buy a soda
        let inserted_money = Money::from_cents(200);
        let result = customer_service.insert_money(machine_id, inserted_money).await;
        assert!(result.is_ok());

        // Act
        let buy_result = customer_service.buy_soda(machine_id, slot_id).await;

        // Assert
        assert!(buy_result.is_ok());
        
        // Check that we got the correct change back
        let remaining_money = customer_service.request_money_back(machine_id).await.unwrap();
        let expected_change = (inserted_money - soda_price).unwrap();
        assert_eq!(remaining_money, expected_change);
    }

    #[tokio::test]
    async fn test_operator_can_refill_slot() {
        // Arrange
        let repository = Arc::new(InMemorySodaMachineRepository::new());
        let operator_service = Arc::new(OperatorService::new(repository.clone()));
        let customer_service = Arc::new(CustomerService::new(repository.clone()));
        
        let machine_id = 1;
        let slot_id = 1;
        let initial_capacity = 5;
        let refill_amount = 3;
        let soda_price = Money::from_cents(150);

        // Create a new machine with a configured slot
        operator_service.create_new_machine(machine_id, 10).await.unwrap();
        
        let cola = Soda::new(
            "Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            soda_price,
            false,
            true,
        ).unwrap();

        operator_service.configure_slot(machine_id, slot_id, initial_capacity, cola.clone()).await.unwrap();
        operator_service.refill_slot(machine_id, slot_id, initial_capacity).await.unwrap();
        
        // Verify initial machine status
        let status = operator_service.get_machine_status(machine_id).await.unwrap();
        assert!(status.contains(&format!("Machine {}", machine_id)), "Status should show machine ID");
        assert!(status.contains("1 slots"), "Status should show number of slots");
        assert!(status.contains(&format!("{} total", initial_capacity)), 
            "Status should show total sodas {}, got: {}", initial_capacity, status);
        assert!(status.contains(&format!("${:.2}", soda_price.as_decimal() * initial_capacity as f64)), 
            "Status should show inventory value for {} sodas, got: {}", initial_capacity, status);

        // Verify initial available sodas
        let available_sodas = customer_service.list_available_sodas(machine_id).await.unwrap();
        assert_eq!(available_sodas.len(), 1);
        assert_eq!(available_sodas[0].slot_id, slot_id);
        assert_eq!(available_sodas[0].soda_name, "Cola");
        assert_eq!(available_sodas[0].price, format!("{:.2}", soda_price.as_decimal()));
        
        // Buy all sodas to empty the slot
        for i in 0..initial_capacity {
            customer_service.insert_money(machine_id, soda_price).await.unwrap();
            customer_service.buy_soda(machine_id, slot_id).await.unwrap();
            
            // Check status after each purchase
            let status = operator_service.get_machine_status(machine_id).await.unwrap();
            let remaining = initial_capacity - (i + 1);
            assert!(status.contains(&format!("{} total", remaining)), 
                "Status should show {} total sodas, status: {}", remaining, status);
            assert!(status.contains(&format!("${:.2}", soda_price.as_decimal() * remaining as f64)),
                "Status should show updated inventory value, got: {}", status);
        }

        // Verify slot is empty by checking available sodas
        let available_sodas = customer_service.list_available_sodas(machine_id).await.unwrap();
        assert_eq!(available_sodas.len(), 0, "Should have no available sodas when slot is empty");

        // Verify empty slot in machine status
        let status = operator_service.get_machine_status(machine_id).await.unwrap();
        assert!(status.contains("0 total"), "Status should show no sodas");
        assert!(status.contains("$0.00 inventory"), "Status should show zero inventory value");

        // Additional verification by trying to buy one more soda
        customer_service.insert_money(machine_id, soda_price).await.unwrap();
        assert!(customer_service.buy_soda(machine_id, slot_id).await.is_err());
        customer_service.request_money_back(machine_id).await.unwrap();

        // Act
        let refill_result = operator_service.refill_slot(machine_id, slot_id, refill_amount).await;

        // Assert
        assert!(refill_result.is_ok());

        // Verify machine status after refill
        let status = operator_service.get_machine_status(machine_id).await.unwrap();
        assert!(status.contains(&format!("{} total", refill_amount)), 
            "Status should show {} total sodas after refill, status: {}", refill_amount, status);
        assert!(status.contains(&format!("${:.2}", soda_price.as_decimal() * refill_amount as f64)),
            "Status should show updated inventory value after refill, got: {}", status);

        // Verify available sodas after refill
        let available_sodas = customer_service.list_available_sodas(machine_id).await.unwrap();
        assert_eq!(available_sodas.len(), 1);
        assert_eq!(available_sodas[0].slot_id, slot_id);
        assert_eq!(available_sodas[0].soda_name, "Cola");
        
        // Verify we can now buy exactly refill_amount sodas
        for _ in 0..refill_amount {
            customer_service.insert_money(machine_id, soda_price).await.unwrap();
            assert!(customer_service.buy_soda(machine_id, slot_id).await.is_ok());
        }

        // Verify slot is empty again by checking available sodas
        let available_sodas = customer_service.list_available_sodas(machine_id).await.unwrap();
        assert_eq!(available_sodas.len(), 0, "Should have no available sodas after buying all refilled sodas");

        // Additional verification by trying to buy one more soda
        customer_service.insert_money(machine_id, soda_price).await.unwrap();
        assert!(customer_service.buy_soda(machine_id, slot_id).await.is_err());
    }
}