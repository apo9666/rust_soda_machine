pub mod domain {
    pub mod value_objects {
        pub mod money;
        pub mod soda;
    }
    pub mod entities {
        pub mod slot;
    }
    pub mod aggregates {
        pub mod soda_machine;
    }
}

pub mod application {
    pub mod customer_service;
    pub mod operator_service;
}

pub mod ports {
    pub mod driving {
        pub mod customer_port;
        pub mod operator_port;
    }
    pub mod driven {
        pub mod soda_machine_repository_port;
    }
}
