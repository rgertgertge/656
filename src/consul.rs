// src/consul.rs
mod consul;

pub use self::consul::ConsulService;

pub mod consul {
    use consul::api::Agent;
    use consul::api::AgentConfig;
    use consul::api::Service;
    use std::sync::Arc;

    pub struct ConsulService {
        agent: Agent,
    }

    impl ConsulService {
        pub fn new(agent_config: AgentConfig) -> Self {
            let agent = Agent::new(agent_config).expect("Failed to create Consul agent");
            ConsulService { agent }
        }

        pub fn register_service(&self, service: Service) -> Result<(), String> {
            self.agent.service_register(service).map_err(|e| format!("Failed to register service: {}", e))
        }

        pub fn deregister_service(&self, service_id: &str) -> Result<(), String> {
            self.agent.service_deregister(service_id).map_err(|e| format!("Failed to deregister service: {}", e))
        }

        pub fn discover_services(&self) -> Result<Vec<Service>, String> {
            self.agent.services().map_err(|e| format!("Failed to discover services: {}", e))
        }
    }
}