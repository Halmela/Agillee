use crate::database::*;
use crate::object::*;
use crate::objects::*;
use crate::edge::*;
use crate::edges::*;
use crate::structure::*;
use postgres::Error;


pub struct Commander {
    db: Database,
}

impl Commander {
    pub fn new(db: Database) -> Commander {
        Commander { db }
    }

    pub fn execute(&mut self, c: Command) -> Result<Structure, Error> {
        match c {
            Command::Read(s) =>
                self.read(s),
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
            _ => Ok(Structure::new(None, None))
        }
    }

    fn read(&mut self, s: Structure) -> Result<Structure, Error> {
        let mut os = vec!();
        for o in s.get_objects() {
			os.push(
    			Objects::from_vec(
        			self.db.query_with_object(&o)?
    			));
        }

        let mut es = vec!();
        
        for e in s.get_edges() {
            es.push(
                Edges::from_vec(
                    self.db.query_with_edge(&e)?
                ));
        }
        println!("{}, {}", os.len(), es.len());

        
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
    Read(Structure)
}
