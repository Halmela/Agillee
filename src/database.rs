
use postgres::{Client, NoTls, Error, Transaction};
//use agillee::table::*;
use crate::table::*;
use std::{thread, time};

//use crate::object::*;


pub fn initialize_db() -> Result<Database, Error> {
	//let schema = Object { id: None, parent: None };
	match Client::connect(
                "postgresql://postgres:psql@postgres:5432/agillee",
                NoTls) {
        Ok(c) => {
            let db = Database {
                client: c,
                tables: vec!(Table::Object, Table::Relation) };
            Ok(db.add_tables()?)},
        Err(_) => {
            println!("can't connect; small wait is in order");
            thread::sleep(time::Duration::from_secs(5));
            initialize_db()
        }
                }
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
                "42P07" => {println!("tables already exist"); Ok(())}, // Error code for creating a duplicate table
                _	    => Err(e)}
        }
    }

    pub fn drop_tables(&mut self) -> Result<(), Error> {
        self.client.execute("DROP TABLE objects, relations", &[])?;
        
        Ok(())
    }
}


