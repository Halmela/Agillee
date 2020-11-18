use postgres::{Client, NoTls, Error, Row, Transaction};
//use agillee::table::*;
use crate::table::*;


pub fn initialize_db() -> Result<Client, Error> {
	let client = Client::connect("host=localhost port=5432 dbname=agillee user=postgres", NoTls)?;
    let db = Database { tables: vec!(Table::Object) };

	Ok(db.add_tables(client)?)
}


pub struct Database {
    tables: Vec<Table>
}

impl Database {
    fn add_tables(&self, mut client: Client) -> Result<Client, Error> {
    	for table in &self.tables {
        	let transaction = client.transaction()?;
        	Database::add_scheme(table_to_scheme(table), transaction)?;
    	}

    	Ok(client)
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

