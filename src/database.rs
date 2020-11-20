use postgres::{Client, NoTls, Error, Row, Transaction};
//use agillee::table::*;
use crate::table::*;
use crate::object::*;


pub fn initialize_db() -> Result<Database, Error> {
	let schema = Object { id: None, parent: None };
    let db = Database {
            client: Client::connect("host=localhost port=5432 dbname=agillee user=postgres", NoTls)?,
            tables: vec!(Table::Object) };

	Ok(db.add_tables()?)
}


pub struct Database {
    //schema: Object,
    tables: Vec<Table>,
    pub client: Client
}

impl Database {
    fn add_tables(mut self) -> Result<Database, Error> {
    	for table in &self.tables {
        	let transaction = self.client.transaction()?;
        	Database::add_scheme(table_to_scheme(table), transaction)?;
    	}

    	Ok(self)
    }

    fn add_scheme(scheme: &str, mut transaction: Transaction) -> Result<(), Error> {
        let res = transaction.execute(scheme, &[]);
    	transaction.commit()?;

        match res {
            Ok(_) 		=> Ok(()),
            Err(e) => match e.code().unwrap().code() {
                "42P07" => Ok(()), // Error code for creating a duplicate table
                _	    => Err(e)}
        }
    }
}

