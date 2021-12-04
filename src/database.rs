use postgres::{Client, NoTls, Error, Transaction};
//use agillee::table::*;
use crate::table::*;
use crate::object::*;
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
    /*
     * Add tables to database.
     */
    fn add_tables(mut self) -> Result<Database, Error> {
    	for table in &self.tables {
        	let transaction = self.client.transaction()?;
        	Database::add_scheme(table_to_scheme(table), transaction)?;
    	}

    	Ok(self)
    }

	/*
     * Create tables to database. Will not error if the tables exist.
     */
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

	/*
     * Drop both tables from database-
     */
    pub fn drop_tables(&mut self) -> Result<(), Error> {
        self.client.execute("DROP TABLE objects, relations", &[])?;
        
        Ok(())
    }

	/*
     * Insert objects to database.
     */
	pub fn insert_objects(&mut self, objects: Vec<Object>) -> Result<(), Error> {
    	let mut transaction = self.client.transaction()?;
    	let statement = transaction.prepare("INSERT INTO Objects (description) VALUES ($1);")?;

    	for o in objects {
        	match o.id {
            	Some(_) => {0},
            	None     => {transaction.execute(&statement, &[&o.description])?}
        	};
    	}

    	
    	Ok(transaction.commit()?)
	}

	/*
     * Insert relations to database.
     */
	pub fn insert_relations(&mut self, relations: Vec<((i32, i32), (Option<bool>, Option<bool>))>) -> Result<(), Error> {
    	let mut transaction = self.client.transaction()?;
    	let stmnt = transaction.prepare(
        	"
            	INSERT INTO Relations AS R (a, b, a2b, b2a)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (a, b) DO UPDATE
                	SET a2b = COALESCE(EXCLUDED.a2b,FALSE) OR R.a2b,
                    	b2a = COALESCE(EXCLUDED.b2a,FALSE) OR R.b2a
            ;")?;

    	for ((a, b), (a2b, b2a)) in relations {
        	if a < b {
                transaction.execute(&stmnt, &[&a, &b, &a2b, &b2a])?;
        	} else {
                transaction.execute(&stmnt, &[&b, &a, &b2a, &a2b])?;
        	}
    	}

        Ok(transaction.commit()?)
	}

	/*
     * Query objects from a list of ids.
     */
    pub fn query_objects(&mut self, ids: Vec<i32>) -> Result<Vec<Object>, Error> {
    	let mut transaction = self.client.transaction()?;
    	let statement = transaction.prepare("SELECT id, description FROM Objects WHERE id = $1;")?;
    	//let mut objs: Vec<(i32, String)> = vec!();

		let mut objects = vec!();
		for id in ids {
    		let obj = transaction.query_one(&statement, &[&id])?;
    		objects.push(Object {
    			id: obj.get("id"),
    			description: obj.get("description") });
		}
		transaction.commit()?;
    	Ok(objects)
	}


    /*
     * Query relations of a given id and relation from database.
     */
    pub fn query_relations(&mut self, id: &i32, rel: Relation) -> Result<Vec<((i32, i32), (Option<bool>, Option<bool>))>, Error> {
        let mut s = String::from("SELECT a, b, a2b, b2a FROM Relations ");
            match rel {
                Relation::Any =>  s.push_str("
                         WHERE ((a = $1) OR (b = $1)) AND (a2b OR b2a);"),
                Relation::Start =>  s.push_str("
                         WHERE (a = $1 AND a2b) OR (b = $1 AND b2a);"),
                Relation::Sink => s.push_str("
                         WHERE (a = $1 AND b2a) OR (a = $1 AND a2b);"),
                Relation::Both => s.push_str("
                         WHERE (a = $1 AND a2b AND b2a) OR (b = $1 AND a2b AND b2a);"),
                Relation::OneWay => s.push_str("
                         WHERE (a = $1 OR b = $1) AND ((a2b AND NOT b2a) OR (b2a AND NOT a2b));"),
                Relation::Closed => s.push_str("
                         WHERE (a = $1 OR b = $1) AND NOT (a2b OR b2a);")
        };
        let statement = self.client.prepare(&s)?;
		Ok(
    		self.client.query(&statement, &[id])?.iter()
                .map(|r| ((r.get("a"), r.get("b")), (r.get("a2b"), r.get("b2a"))))
                .collect()
		)

    }
}
