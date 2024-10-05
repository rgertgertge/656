// src/mpc.rs
mod mpc;

pub use self::mpc::MPC;

pub mod mpc {
    use mpc::MPCProtocol;
    use std::sync::Arc;

    pub struct MPC {
        protocol: MPCProtocol,
    }

    impl MPC {
        pub fn new() -> Self {
            let protocol = MPCProtocol::new();
            MPC { protocol }
        }

        pub fn compute_secure_sum(&self, inputs: &[u64]) -> Result<u64, String> {
            self.protocol.compute_secure_sum(inputs).map_err(|e| format!("Failed to compute secure sum: {}", e))
        }

        pub fn compute_secure_product(&self, inputs: &[u64]) -> Result<u64, String> {
            self.protocol.compute_secure_product(inputs).map_err(|e| format!("Failed to compute secure product: {}", e))
        }
    }
}