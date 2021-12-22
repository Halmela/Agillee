use crate::database::*;
use crate::object::*;
use crate::edge::*;
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
            Command::ReadObject(o) =>
                self.db.query_with_object(o)
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
}


pub enum Command {
    CreateObject(Object),
    //Update,
    //Delete,
    ReadObject(Object)
}
