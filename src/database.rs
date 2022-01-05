use postgres::{Client, NoTls, Error, Transaction};
//use agillee::table::*;
use crate::table::*;
use crate::object::*;
use crate::objects::*;
use crate::edge::*;
use std::{thread, time};

//use crate::object::*;


/*
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
                client: c };
            db.add_tables()?;
            Ok(db)},
        Err(_) => {
            println!("can't connect; small wait is in order");
            thread::sleep(time::Duration::from_secs(5));
            initialize_db()
        }
    }
}
*/


pub struct Database {
    //schema: Object,
    //pub client: Client
}

impl Database {
    /*
     * Add tables to database.
     */
     /*
    fn add_tables(&mut self) -> Result<(), Error> {
    	for scheme in Table::all_schemes() {
        	let transaction = self.client.transaction()?;
        	Database::add_scheme(scheme, transaction)?;
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
     * Drop tables from database
     */
    pub fn drop_tables(&mut self) -> Result<(), Error> {
        self.client.execute("DROP TABLE objects, relations, edges, forms, formations", &[])?;
        
        Ok(())
    }
    */


	pub fn create_edge(mut t: Transaction, e: Edge) -> Result<Option<Edge>, Error> {
    	let edge = Database::insert_edge(e, t.transaction()?)?;
    	if edge.is_some() {
        	println!("edge addition was success");
            t.commit()?;
            Ok(edge)
    	} else {
        	println!("edge addition was failure");
            Ok(None)
    	}

	}

    pub fn create_object(mut t: Transaction, object: Object) -> Result<Option<(Object, Edge)>, Error> {
		let obj  = Database::insert_object(object, t.transaction()?)?;
        let obj  = Database::upsert_form(obj, t.transaction()?)?;
        let edge = Database::insert_root(&obj, t.transaction()?)?;

		if let (Some(o), Some(e)) = (obj, edge) {
    		println!("object addition was succesful");
    		t.commit()?;
    		Ok(Some((o, e)))
		} else {
    		println!("object addition failed");
            Ok(None)
		}
    }
    

    fn upsert_form(object: Option<Object>, mut transaction: Transaction) -> Result<Option<Object>, Error> {
        let form_upsert = "INSERT INTO Formations (object, form)
        				   VALUES ($1, $2)
            			   ON CONFLICT (object) DO UPDATE
                			   SET object = EXCLUDED.object,
                			       form   = EXCLUDED.form
                			";
        if let Some(o) = object {
            match transaction.execute(form_upsert, &[&o.get_id(), &o.get_form_id()])? {
                1 => { println!("form updated");       transaction.commit()?; Ok(Some(o))   }
                _ => { println!("form update failed");  Ok(None)}
            }
        } else { println!("form update failed");  Ok(None)}
    }

    fn insert_object(o: Object, mut transaction: Transaction) -> Result<Option<Object>, Error> {
        let obj_insert = "INSERT INTO Objects (description)
             					VALUES ($1)
             					RETURNING id;";
		let mut object = None;
        if let (Some(desc), Some(_)) = (o.description, &o.form) {
            let o_id: i32 = transaction.query_one(obj_insert, &[&desc])?.get("id");
            object = Some(Object::new(Some(&o_id), Some(desc), o.form, o.root));
        }

        if object.is_some() {
            println!("object added");
    		transaction.commit()?;
        } else {
            println!("object addition failed");
        }

		Ok(object)
    }


    fn insert_edge(e: Edge, mut transaction: Transaction) -> Result<Option<Edge>, Error> {
        let edge_insert = "INSERT INTO Edges ( a, b, a2b, b2a)
            				VALUES           ($1,$2, $3,  $4)
                			ON CONFLICT DO NOTHING
                    		RETURNING a, b, a2b, b2a";

		let mut edge = None;

        if let (Some(a), Some(b), Some(a2b), Some(b2a)) = (e.a, e.b, e.a2b, e.b2a) {
            if a > 4 || b > 4 {
                let edge_row = transaction.query_one(edge_insert, &[&a, &b, &a2b, &b2a])?;
                edge = Some(Edge::new(edge_row.get("a"), edge_row.get("b"), edge_row.get("a2b"), edge_row.get("b2a")));
            } else {
                println!("don't touch the core");
            }
        }

        if edge.is_some() {
            println!("edge added");
            transaction.commit()?;
        } else {
            println!("edge addition failed");
        }

		Ok(edge)
    }


    fn insert_root(obj: &Option<Object>, transaction: Transaction) -> Result<Option<Edge>, Error> {
    	if let Some(e) = Edge::root(obj) {
            Database::insert_edge(e, transaction)
    	} else {
        	Ok(None)
    	}
    }


	pub fn query_with_object(mut t: Transaction, object: &Object) -> Result<Vec<Object>, Error> {
    	let query: &'static str  = "
        	SELECT O.id, O.description, F.form, COALESCE(NULLIF(E.a, O.id), E.b) as root
            FROM Objects O LEFT JOIN Edges E ON O.id = E.a OR O.id = E.b
                		   JOIN Formations F ON O.id = F.object
            WHERE   COALESCE($1 = O.id, TRUE)
                AND COALESCE($2 LIKE O.description, TRUE)
                AND COALESCE($3 = F.form, TRUE)
				AND COALESCE(
    					(O.id = E.a AND (E.a2b = 4 AND E.b2a = E.b) AND COALESCE(E.b = $4, TRUE))
        			 OR (O.id = E.b AND (E.b2a = 4 AND E.a2b = E.a) AND COALESCE(E.a = $4, TRUE))
        			, TRUE)
            	";

		let res = t.query(query, &[ &object.get_id(),
                            		&object.get_description(),
                            		&object.get_form_id(),
                            		&object.get_root()])?
                    .iter()
                    .map(|row|
                		Object::new(
                    		row.try_get("id").ok().as_ref(),
                        	row.try_get("description").ok(),
                            Form::from_id(row.try_get("form").ok()),
                            row.try_get("root").ok()))
                    .collect();
        t.commit()?;
        Ok(res)
	}

	pub fn query_with_edge(mut t: Transaction, edge: &Edge) -> Result<Vec<Edge>, Error> {
    	let query: &'static str  = "
        	SELECT a, b, a2b, b2a
            FROM Edges 
            WHERE   COALESCE($1 = a, TRUE)
                AND COALESCE($2 = b, TRUE)
                AND COALESCE($3 = a2b, TRUE)
				AND COALESCE($4 = b2a, TRUE)
            	";

		let res = t.query(query, &[ &edge.get_a(),
                            		&edge.get_b(),
                            		&edge.get_a2b(),
                            		&edge.get_b2a()])?
                    .iter()
                    .map(|row|
                		Edge::new(
                    		row.try_get("a").ok(),
                        	row.try_get("b").ok(),
                            row.try_get("a2b").ok(),
                            row.try_get("b2a").ok()))
                    .collect();
        t.commit()?;
        Ok(res)
	}
}
