use postgres::{Error};
use agillee::objects::*;
use agillee::database::*;
use agillee::cli::*;


fn main() -> Result<(), Error> {
    CLI::new(Objects::new(initialize_db()?)?)
        .start()?;

    Ok(())
}


