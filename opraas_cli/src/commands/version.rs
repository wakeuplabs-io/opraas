

pub async fn version() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}