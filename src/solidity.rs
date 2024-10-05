// src/solidity.rs
mod solidity;

pub use self::solidity::SolidityContract;

pub mod solidity {
    use solc::CompilerInput;
    use solc::CompilerOutput;
    use std::fs::File;
    use std::io::Read;

    pub struct SolidityContract {
        abi: String,
        bytecode: String,
    }

    impl SolidityContract {
        pub fn new(source_code: &str) -> Result<Self, String> {
            let input = CompilerInput::sources(vec![source_code]);
            let output = solc::compile(&input).map_err(|e| format!("Compilation error: {}", e))?;

            let contract_name = output.contracts.keys().next().ok_or("No contracts found")?;
            let contract_output = output.contracts.get(contract_name).ok_or("No contract output found")?;

            let abi = contract_output.abi.clone().ok_or("No ABI found")?;
            let bytecode = contract_output.evm.bytecode.object.clone().ok_or("No bytecode found")?;

            Ok(SolidityContract { abi, bytecode })
        }
    }
}