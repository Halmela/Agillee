use crate::data::database::*;
use crate::models::object::*;
use crate::models::objects::*;
use crate::models::edges::*;
use crate::models::structure::*;
use postgres::{Transaction, Error};

enum E { F }

pub struct Commander {
}

impl Commander {
    /*
    pub fn new(db: Database) -> Commander {
        Commander { db }
    }
    */

    pub fn execute(t: Transaction, c: Command) -> Result<Structure, Error> {
        match c {
            Command::Read(s) =>
                Commander::read(t, s),
            Command::Create(s) =>
                Commander::create(t, s),
                /*
            Command::ReadObject(o) =>
                self.db.query_with_object(&o)
                    .map(|v| Structure::new(Some(v), None)),
            Command::CreateObject(o) =>
                self.db.create_object(o)
                    .map(|oe| oe.map_or_else(
                    	|| Structure::new(None, None),
                    	|(o,e)| Structure::new(
                        	Some(vec!(o)),
                        	Some(vec!(e))
                    	))),
                    	*/
            Command::Init => Commander::init(t),
            _ => Ok(Structure::new(None, None))
        }
    }

    pub fn init(t: Transaction) -> Result<Structure, Error>  {
        //Database::drop_tables(t)?;
        Database::add_tables(t)?;

		Ok(Structure::empty())
    }

    pub fn create(mut t: Transaction, s: Structure) -> Result<Structure, Error> {
        let mut os = vec!();
        let mut es = vec!();

        for object in s.get_objects() {
			if let Some((o,e)) = Database::create_object(t.transaction()?, object)? {
    			os.push(o);
    			es.push(e);
			} else {
    			return Ok(Structure::empty());
			}
        }

        for edge in s.get_edges() {
            if let Some(e) = Database::create_edge(t.transaction()?, edge)? {
                es.push(e);
            } else {
    			return Ok(Structure::empty());
			}
        }
        

        t.commit()?;
        Ok(Structure::from_structs(Objects::from(os), Edges::from(es)))
    }

    pub fn read(mut t: Transaction, s: Structure) -> Result<Structure, Error> {
        let mut os = vec!();
        for o in s.get_objects() {
			os.push(
    			Objects::from_vec(
        			Database::query_with_object(t.transaction()?, &o)?
    			));
        }

        let mut es = vec!();
        
        for e in s.get_edges() {
            es.push(
                Edges::from_vec(
                    Database::query_with_edge(t.transaction()?, &e)?
                ));
        }
        
        let es = es.iter()
            .fold(
                Edges::empty(),
                |acc, e| acc.merge(e)
            );

        let os = os.iter()
            .fold(
                Objects::empty(),
                |acc, o| acc.merge(o)
            );

        t.commit()?;

        Ok(
            Structure::from_structs(os, es)
        )
    }
    
}


pub enum Command {
    CreateObject(Object),
    //Update,
    //Delete,
    ReadObject(Object),
    Read(Structure),
    Create(Structure),
    Init
}
