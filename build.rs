use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("KelpDeposits", "abi/kelp_deposits.json")?
        .generate()?
        .write_to_file("src/abi/kelpDeposits.rs")?;

    Ok(())
}
