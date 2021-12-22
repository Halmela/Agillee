use postgres::{Error};
use agillee::objects::*;
use agillee::database::*;
use agillee::cli::*;
use agillee::commander::*;


fn main() -> Result<(), Error> {
    CLI::new(Commander::new(initialize_db()?))
        .start()?;

    Ok(())
}


