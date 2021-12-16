//use rand::Rng;
use postgres::{Error};
use agillee::objects::*;
//use agillee::table::*;
use agillee::database::*;
use agillee::cli::*;


fn main() -> Result<(), Error> {
    let db = initialize_db()?;
    let objs = Objects::new(db);
    let mut cli = CLI::new(objs);

    cli.start()?;

    Ok(())
}


