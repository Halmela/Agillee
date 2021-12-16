use postgres::{Client, NoTls, Error, Transaction};
//use agillee::table::*;
use crate::table::*;
use crate::object::*;
use crate::objects::*;
use std::{thread, time};

//use crate::object::*;


pub fn initialize_db() -> Result<Database, Error> {
    /*
     * "host=localhost port=5432 dbname=agillee user=postgres"
     * "postgresql://postgres:psql@postgres:5432/agillee"
     */

	match Client::connect(
                "host=localhost port=5432 dbname=agillee user=postgres",
                NoTls) {
        Ok(c) => {
            let mut db = Database {
                client: c,
                tables: vec!(Table::Object, Table::Relation) };
            db.add_tables()?;
            Ok(db)},
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
    fn add_tables(&mut self) -> Result<(), Error> {
    	for table in &self.tables {
        	let transaction = self.client.transaction()?;
        	Database::add_scheme(table_to_scheme(table), transaction)?;
            
    	}
    	Ok(())
    }

	/*
     * Create tables to database. Will not error if the tables exist.
     */
    fn add_scheme(scheme: &str, mut transaction: Transaction) -> Result<(), Error> {
        let res = transaction.batch_execute(scheme);
    	transaction.commit()?;

        match res {
            Ok(_) 		=> Ok(()),
            Err(e) => match e.code().unwrap().code() {
                "42P07" => {Ok(())}, // Error code for creating a duplicate table
                _	    => Err(e)}
        }
    }

	/*
     * Drop both tables from database
     */
    pub fn drop_tables(&mut self) -> Result<(), Error> {
        self.client.execute("DROP TABLE objects, relations", &[])?;
        
        Ok(())
    }

	/*
     * Insert objects to database.
     */
	pub fn insert_objects(&mut self, objects: Vec<Object>) -> Result<Vec<Object>, Error> {
    	let mut transaction = self.client.transaction()?;
    	let statement = transaction.prepare(
        	"INSERT INTO Objects (description)
             VALUES ($1)
             RETURNING *;")?;

		let mut os = vec!();
    	for o in objects {
        	match o.id {
            	Some(_) => {},
            	None     => {
                	let obj = transaction.query_one(&statement, &[&o.description])?;
                	os.push( Object {
                    			id: obj.get("id"),
                    			description: obj.get("description") });
            	}
        	};
    	}
        transaction.commit()?;
    	Ok(os)
	}

	pub fn insert_relation(&mut self, mut a: i32, mut b: i32, mut rel: Relation) -> Result<(), Error> {
    	let mut transaction = self.client.transaction()?;
    	let stmnt = transaction.prepare(
        	"
            	INSERT INTO Relations AS R (a, b, a2b, b2a)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (a, b) DO UPDATE
                	SET a2b = EXCLUDED.a2b,
                    	b2a = EXCLUDED.b2a
                RETURNING a,b,a2b,b2a
            ;")?;

        // a and b need to swap places if their ids are not ordered
        if a > b {
            let t = a;
            a = b;
            b = t;

            if let Relation::Sink(c) = rel {
                rel = Relation::Start(c);
            } else if let Relation::Start(c) = rel {
                rel = Relation::Sink(c);
            }
        }

        let none: Option<bool> = None;

        match rel {
        	Relation::Start(c) => { transaction.query(&stmnt,&[&a,&b, &c,           &none])?        ;},
        	Relation::Sink(c)  => { transaction.query(&stmnt,&[&a,&b, &none,        &c])?           ;},
        	Relation::Both     => { transaction.query(&stmnt,&[&a,&b, &Some(true),  &Some(true)])?  ;},
        	Relation::Closed   => { transaction.query(&stmnt,&[&a,&b, &Some(false), &Some(false)])? ;},
        	Relation::Empty    => { transaction.query(&stmnt,&[&a,&b, &none,        &none])?        ;},
        	_                  => {},
        };
        transaction.commit()?;

    	Ok(())
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
                	SET a2b = COALESCE(EXCLUDED.a2b,R.a2b),
                    	b2a = COALESCE(EXCLUDED.b2a,R.b2a)
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

	pub fn query_all_objects(&mut self) -> Result<Vec<Object>, Error> {
		let mut objects = vec!();
		for row in self.client.query("SELECT * FROM Objects", &[])? {
    		objects.push(Object {
    			id: row.get("id"),
    			description: row.get("description") });
		}
		Ok(objects)
	}

	pub fn query_all_relations(&mut self) -> Result<Vec<((i32,i32),(Option<bool>,Option<bool>))>, Error> {
		let mut relations = vec!();
		for row in self.client.query("SELECT * FROM Relations", &[])? {
    		relations.push((
    			(row.get("a"),
    			row.get("b")),
    			(row.get("a2b"),
    			row.get("b2a")),
    		));
		}
		Ok(relations)
	}


    /*
     * Query relations of a given id and relation from database.
     */
    pub fn query_relations(&mut self, id: &i32, rel: Relation) -> Result<Vec<((i32, i32), (Option<bool>, Option<bool>))>, Error> {
        let mut s = String::from("SELECT a, b, a2b, b2a FROM Relations WHERE ");
        s.push_str(&relation_as_condition(rel));
        let statement = self.client.prepare(&s)?;
		Ok(
    		self.client.query(&statement, &[id])?.iter()
                .map(|r| ((r.get("a"), r.get("b")), (r.get("a2b"), r.get("b2a"))))
                .collect()
		)
    }


}


fn relation_as_condition(rel: Relation) -> String {
    let f = |b| {
        if let Some(a) = b {
            if a {"TRUE"} else {"FALSE"}
        } else { "NULL" }};

    match rel {
        Relation::Any      => {return "((a = $1) OR (b = $1));".to_string() },
        Relation::Start(c) => {return "(a = $1 AND (a2b=REP) AND NOT COALESCE(b2a,FALSE))\
            			            OR (b = $1 AND (b2a=REP) AND NOT COALSSCE(a2b,FALSE));"
                			    .replace("REP",f(c.clone())) },
        Relation::Sink(c)  => {return "(a = $1 AND (b2a=REP) AND NOT COALESCE(a2b,FALSE))\
            			            OR (a = $1 AND (a2b=REP) AND NOT COALSSCE(b2a,FALSE));"
                			    .replace("REP",f(c.clone())) },
        Relation::Both     => {return "(a = $1 OR b = $1) AND a2b AND b2a;".to_string() },
        Relation::OneWay   => {return "(a = $1 OR b = $1)\
            			          AND ((a2b AND NOT COALESCE(b2a,FALSE))
                    			    OR (b2a AND NOT COALESCE(a2b,FALSE)));".to_string() },
        Relation::Closed   => {return "(a = $1 OR b = $1) AND NOT COALESCE(a2b OR b2a,FALSE);".to_string() },
        Relation::Empty    => {return "(a = $1 OR b = $1) AND (a2b = NULL AND b2a = NULL);".to_string() }
   }
}
