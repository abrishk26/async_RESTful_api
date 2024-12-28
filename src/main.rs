mod logger;
mod models;
mod db;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenvy::dotenv()?;
    logger::init()?;

    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}
