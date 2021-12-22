use crate::database::*;
use crate::objects::*;
use crate::object::*;
use crate::edge::*;
use std::collections::{HashSet};
use std::fmt;
use std::fmt::Write;
use postgres::Error;
use itertools::Itertools;


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




pub struct Objects_n {
    objects: HashSet<Object>
}

impl Objects_n {
    pub fn from_vec(v: Vec<Object>) -> Objects_n {
        let mut hs = HashSet::new();
        for o in v {
            hs.insert(o);
        }

        Objects_n {objects: hs}
    }
}

impl fmt::Display for Objects_n {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.objects.is_empty() {
        	write!(&mut res, "No edges\n").unwrap();
    	} else {
        	for o in self.objects.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string()).unwrap();
        	}

    	}
        write!(f, "{}", res)
	}
}



pub struct Edges {
    edges: HashSet<Edge>
}

impl Edges {
    pub fn from_vec(v: Vec<Edge>) -> Edges {
        let mut hs = HashSet::new();
        for e in v {
            hs.insert(e);
        }

        Edges { edges: hs }
    }
}
impl fmt::Display for Edges {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.edges.is_empty() {
        	write!(&mut res, "No objects\n").unwrap();
    	} else {
        	for o in self.edges.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string()).unwrap();
        	}

    	}
        write!(f, "{}", res)
	}
}


pub struct Structure {
    objects: Objects_n,
    edges:   Edges
}

impl fmt::Display for Structure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();
    	write!(&mut res, "{}\n", self.objects);
    	write!(&mut res, "{}\n", self.edges);

    	write!(f, "{}", res)
	}
}

impl Structure {
    pub fn new(os: Option<Vec<Object>>, es: Option<Vec<Edge>>) -> Structure {
        Structure {
            objects: Objects_n::from_vec(os.unwrap_or_default()),
			edges:   Edges::from_vec(es.unwrap_or_default())
        }
    }
}

pub enum Command {
    CreateObject(Object),
    //Update,
    //Delete,
    ReadObject(Object)
}
