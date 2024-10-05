// src/event_listener.rs
mod event_listener;

pub use self::event_listener::EventListener;

pub mod event_listener {
    use web3::types::{Event, FilterBuilder};
    use web3::Web3;
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    pub struct EventListener {
        web3: Web3<web3::transports::Http>,
    }

    impl EventListener {
        pub fn new(endpoint: &str) -> Self {
            let transport = web3::transports::Http::new(endpoint).expect("Failed to create HTTP transport");
            let web3 = Web3::new(transport);
            EventListener { web3 }
        }

        pub async fn listen_for_events(&self, contract_address: String, event_name: String) -> Result<Vec<Event>, String> {
            let filter = FilterBuilder::new()
                .address(vec![contract_address])
                .topics(vec![])
                .build();

            let events = self.web3.eth_subscribe(filter).await.map_err(|e| format!("Failed to subscribe to events: {}", e))?;

            let mut collected_events = Vec::new();

            while let Some(event) = events.next().await {
                match event {
                    Ok(e) => {
                        if e.topics.first().unwrap().to_hex() == event_name {
                            collected_events.push(e);
                        }
                    },
                    Err(err) => {
                        return Err(format!("Failed to process event: {}", err));
                    }
                }
            }

            Ok(collected_events)
        }
    }
}